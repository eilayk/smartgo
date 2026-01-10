use anyhow::anyhow;
use anyhow::Context;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::database::HasValueRef;
use sqlx::{Database, Decode, FromRow, Pool, Sqlite};
use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

use crate::go_api::GoApi;

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Sqlite>,
    pub api: GoApi,
}

// DATABASE AND RESPONSE STRUCTS
#[derive(FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Stop {
    stop_id: String,
    stop_name: String,
}

#[derive(FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteSearch {
    route_name: String,
    route_id: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StopTimeResponse {
    pub stop_name: String,
    pub route_name: String,
    pub stop_times: Vec<StopTime>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StopTime {
    pub arrival_time: Time,
    pub headsign: String,
    pub trip_number: String,
    pub scheduled_platform: Option<String>,
    pub actual_platform: Option<String>,
    pub actual_arrival_time: Option<Time>,
    pub train_length: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeQuery {
    pub day: String, // format: "yyyymmdd"
    pub time: Time,
}

// CUSTOM STRUCTS
#[derive(Debug, Clone, PartialEq)]
pub struct Time {
    hour: u8,
    minute: u8,
    second: u8,
}

impl Time {
    pub fn is_relevant_to(&self, t: &Time) -> bool {
        // time is relevant if it's after, or within three minutes
        let self_duration = self.to_duration();
        let t_duration = t.to_duration();
        self_duration > t_duration
            || (t_duration - self_duration) < std::time::Duration::new(60 * 3, 0)
    }

    // determine if cached value should be used
    pub fn should_use_cached(&self, t: &Time) -> bool {
        // use cached value if less than 2 minutes have passed since last request
        let self_duration = self.to_duration();
        let t_duration = t.to_duration();
        (self_duration - t_duration) <= std::time::Duration::new(60 * 2, 0)
    }

    fn to_duration(&self) -> std::time::Duration {
        let hour = self.hour as u64;
        let minute = self.minute as u64;
        let second = self.second as u64;
        let seconds: u64 = hour * 60 * 60 + minute * 60 + second;
        std::time::Duration::new(seconds, 0)
    }

    fn num_to_str(num: u8) -> String {
        if num < 10 {
            format!("0{}", num)
        } else {
            format!("{}", num)
        }
    }
}

impl FromStr for Time {
    type Err = AppError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = "format time as 00:00:00";
        let parts: Vec<&str> = s.split(':').collect();
        let hour: u8 = parts
            .get(0)
            .context(err)?
            .parse()
            .context("failed to parse hour")?;
        let minute: u8 = parts
            .get(1)
            .context(err)?
            .parse()
            .context("failed to parse minute")?;
        let second: u8 = parts
            .get(2)
            .context(err)?
            .parse()
            .context("failed to parse second")?;
        Ok(Time {
            hour: hour,
            minute: minute,
            second: second,
        })
    }
}

// implement custom deserializer for serde
// TODO: consider replacing this with serde_as crate
impl<'de> Deserialize<'de> for Time {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        Time::from_str(&s).map_err(|e| serde::de::Error::custom(e))
    }
}

impl Serialize for Time {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(self)
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hour = Time::num_to_str(self.hour);
        let minute = Time::num_to_str(self.minute);
        let second = Time::num_to_str(self.second);
        write!(f, "{}:{}:{}", hour, minute, second)
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct DateTime {
    pub date: String,
    pub time: Time,
}

impl FromStr for DateTime {
    type Err = AppError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        let date = parts.get(1).context("failed to parse date")?;
        let time = Time::from_str(parts.get(1).context("failed to parse time")?)?;

        Ok(DateTime {
            date: date.to_string(),
            time,
        })
    }
}

impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        DateTime::from_str(&s).map_err(|e| serde::de::Error::custom(e))
    }
}

impl Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}:{}:{}",
            self.date, self.time.hour, self.time.minute, self.time.second
        )
    }
}

// TODO: probably delete this
// DB is the database driver
// `'r` is the lifetime of the `Row` being decoded
impl<'r, DB: Database> Decode<'r, DB> for Time
where
    // we want to delegate some of the work to string decoding so let's make sure strings
    // are supported by the database
    &'r str: Decode<'r, DB>,
{
    fn decode(
        value: <DB as HasValueRef<'r>>::ValueRef,
    ) -> Result<Time, Box<dyn Error + 'static + Send + Sync>> {
        // the interface of ValueRef is largely unstable at the moment
        // so this is not directly implementable

        // however, you can delegate to a type that matches the format of the type you want
        // to decode (such as a UTF-8 string)

        let value = <&str as Decode<DB>>::decode(value)?;

        // now you can parse this into your type (assuming there is a `FromStr`)
        match value.parse() {
            Ok(time) => Ok(time),
            Err(err) => Err(anyhow!(err).into()),
        }

        // Ok(anyhow!(value.parse())?)
    }
}

// create AppError that can return errors from handlers
#[derive(Debug)]
pub struct AppError(anyhow::Error);
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(format!("Failed with error: {}", self.0)),
        )
            .into_response()
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

pub fn get_trip_number_from_id(trip_id: &str) -> Result<&str, AppError> {
    let v: Vec<&str> = trip_id.split('-').collect();
    Ok(v.get(2).context("trip_id format is invalid")?)
}
