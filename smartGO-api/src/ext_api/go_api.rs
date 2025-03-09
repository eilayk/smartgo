use std::{time::Duration, fmt::Display, sync::{Arc, atomic::AtomicBool}};
use rustc_hash::FxHashMap;
use crate::models::AppError;

use super::response_structs::{NextServiceResponse, Line, ServiceAtAGlanceResponse, Trip, Trips};
use mini_moka::sync::Cache;

type StopId = String;
type RouteId = String;
type TripNumber = String;
// TODO: explore caching using thruple tuple type (avoid cloning entire hashmap)
type NextServiceLines = Arc<FxHashMap<(RouteId, TripNumber), Line>>;

#[derive(Clone)]
struct NextServiceCache(Cache<StopId, NextServiceLines>);
impl NextServiceCache {
    pub fn new() -> Self {
        Self(Cache::builder().time_to_live(Duration::from_secs(60 * 10)).build())
        // Self(Cache::builder().time_to_live(Duration::from_secs(60 * 2)).build())
    }

    pub fn get_cached(&self, stop_id: &str, route_id: &str, trip_number: &str) -> Option<Option<Line>> {
        let cache_key =  stop_id.to_string();
        let cache_key_2 = (route_id.to_string(), trip_number.to_string());
        self.0.get(&cache_key).map(|map| map.get(&cache_key_2).cloned())
        // TODO: figure out a way to avoid cloning here
    }

    pub fn cache_next_service(&self, stop_id: &str, route_id: &str, trip_number: &str, next_service: Option<NextServiceResponse>) -> Option<Line> {
        let mut map: FxHashMap<(RouteId, TripNumber), Line> = FxHashMap::default();
        if let Some(next_service) = next_service {
            for line in next_service.next_service.lines.iter() {
                map.insert((line.line_code.clone(), line.trip_number.clone()), line.clone());
            }
        }
        let map = Arc::new(map);
        self.0.insert(stop_id.to_string(), Arc::clone(&map));
        self.get_cached(stop_id, route_id, trip_number).and_then(|op| op)
    }
}

#[derive(Clone)]
struct ServiceAtaGlanceCache(Cache<TripNumber, Trip>);
impl ServiceAtaGlanceCache {
    pub fn new() -> Self {
        Self(Cache::builder().time_to_live(Duration::from_secs(60 * 10)).build())
    }

    pub fn get_cached(&self, trip_number: &str) -> Option<Trip> {
        let cache_key = trip_number.to_string();
        self.0.get(&cache_key)
    }

    pub fn cache_service_at_a_glance(&self, trips: Trips) {
        for trip in trips.trip.iter() {
            self.0.insert(trip.trip_number.clone(), trip.clone());
        }
    }
}

#[derive(Clone)]
pub struct GoApi {
    api_key: String,
    client: reqwest::Client,
    next_service_cache: NextServiceCache,
    service_at_a_glance_cache: ServiceAtaGlanceCache,
    service_at_a_glance_updated: Arc<AtomicBool>,
}

impl GoApi {
   pub fn new(api_key: &str) -> GoApi {
        GoApi {
            api_key: String::from(api_key),
            client: reqwest::Client::new(),
            next_service_cache: NextServiceCache::new(),
            service_at_a_glance_cache: ServiceAtaGlanceCache::new(),
            service_at_a_glance_updated: Arc::new(AtomicBool::new(false))
        }
    }

    pub fn get_request_path<S: AsRef<str> + Display>(&self, s: S) -> String {
        format!("https://api.openmetrolinx.com/OpenDataAPI/api/V1/{}?key={}", s, self.api_key)
    }

    pub async fn get_next_service(&self, stop_id: &str, route_id: &str, trip_number: &str) -> Option<Line> {
        match self.next_service_cache.get_cached(stop_id, route_id, trip_number) {
            Some(op) => op,
            None => {
                // TODO: update to handle error handling
                self.get_next_service_from_go(stop_id, route_id, trip_number).await.ok().flatten()
            }
        }
    }

    async fn get_next_service_from_go(&self, stop_id: &str, route_id: &str, trip_number: &str) -> Result<Option<Line>, AppError> {
        tracing::debug!("get next service from go for stop_id: {}", stop_id);
        let path = self.get_request_path(format!("Stop/NextService/{}", &stop_id));
        let resp: Option<NextServiceResponse> = self.client.get(path).send().await?.json().await.ok();
        Ok(self.next_service_cache.cache_next_service(stop_id, route_id, trip_number, resp))
    }

    async fn get_service_at_a_glance_from_go(&self) -> Result<(), AppError> {
        tracing::debug!("get service at a glance from go");
        let path = self.get_request_path("ServiceataGlance/Trains/All/");
        let resp: ServiceAtAGlanceResponse = self.client.get(path).send().await?.json().await?;
        self.service_at_a_glance_cache.cache_service_at_a_glance(resp.trips);
        self.service_at_a_glance_updated.store(true, std::sync::atomic::Ordering::Release);
        // figure out a way to spawn a task that changes this to false after 10 minutes
        tokio::spawn(GoApi::update_service_at_a_glance(self.service_at_a_glance_updated.clone()));
        Ok(())        
    }

    async fn update_service_at_a_glance(bool: Arc<AtomicBool>) {
        tokio::time::sleep(Duration::from_secs(60 * 10)).await;
        bool.store(false, std::sync::atomic::Ordering::Release);
    }

    pub async fn get_service_at_a_glance(&self, trip_number: &str) -> Option<Trip> {
       let up_to_date =  self.service_at_a_glance_updated.load(std::sync::atomic::Ordering::Acquire);
        if !up_to_date {
            // update cached service_at_a_glance
            let _ = self.get_service_at_a_glance_from_go().await.map_err(|op| tracing::error!("error: {:?}", op));
        }

        self.service_at_a_glance_cache.get_cached(trip_number)
    }

}