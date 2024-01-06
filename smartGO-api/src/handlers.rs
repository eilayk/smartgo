use sqlx::Row;
use axum::{
    Json, extract::{State, Path, Query}, debug_handler
};
use std::str::FromStr;
use crate::models::{Stop, AppError, RouteSearch, StopTime, TimeQuery, Time, AppState, get_trip_number_from_id, StopTimeResponse, get_api_route_from_route_id};

#[debug_handler]
pub async fn get_stops(
    State(AppState { db, ..}): State<AppState>,
) -> Result<Json<Vec<Stop>>, AppError> {
    let resp = sqlx::query_as::<_, Stop>("
        SELECT * FROM stops
        ")
        .fetch_all(&db)
        .await?;
    Ok(Json(resp))
}

#[debug_handler]
pub async fn get_stop(
    State(AppState { db, ..}): State<AppState>,
    Path(stop_id): Path<String>,
) -> Json<Stop> {
    let resp = sqlx::query_as::<_, Stop>("
        SELECT * FROM stops WHERE stop_id = ?
        ")
        .bind(stop_id)
        .fetch_one(&db)
        .await.unwrap();
        Json(resp)
}

pub async fn search_for_stop(
    State(AppState { db, ..}): State<AppState>,
    Path(stop_name): Path<String>,
) -> Result<Json<Vec<Stop>>, AppError> {
    let arg = format!("%{}%", stop_name);
    let resp = sqlx::query_as::<_, Stop>(
        "SELECT stop_id, stop_name FROM stops WHERE stop_name LIKE ? LIMIT 15"
    ).bind(arg)
    .fetch_all(&db)
    .await?;
    Ok(Json(resp))
}

pub async fn get_all_routes(
    State(AppState { db, ..}): State<AppState>,
) -> Result<Json<Vec<RouteSearch>>, AppError> {
    let resp = sqlx::query_as::<_, RouteSearch>("
        SELECT DISTINCT routes.long_name as route_name, routes.route_id from routes
    ")
    .fetch_all(&db)
    .await?;
    Ok(Json(resp))
}

pub async fn get_route_from_stop(
    // TODO: update db to only show train stops
    State(AppState { db, ..}): State<AppState>,
    Path(stop_id): Path<String>,
) -> Result<Json<Vec<RouteSearch>>, AppError> {
    let resp = sqlx::query_as::<_, RouteSearch>("
        SELECT DISTINCT routes.long_name as route_name, routes.route_id from routes
            JOIN trips ON trips.route_id = routes.route_id
            JOIN stop_times ON stop_times.trip_id = trips.trip_id
            WHERE stop_times.stop_id = ?
    ").bind(stop_id)
    .fetch_all(&db)
    .await?;
    Ok (Json(resp))
}

pub async fn get_ordered_stops_for_route(
    State(AppState { db, ..}): State<AppState>,
    Path(route_id): Path<String>
) -> Result<Json<Vec<Stop>>, AppError> {
    let resp = sqlx::query_as("
        SELECT DISTINCT stops.stop_id, stops.stop_name 
            FROM trips
            INNER JOIN stop_times ON stop_times.trip_id = trips.trip_id
            INNER JOIN stops ON stops.stop_id = stop_times.stop_id
            WHERE route_id = ?
    ")
    .bind(route_id)
    .fetch_all(&db)
    .await?;
    Ok(Json(resp))
}

pub async fn get_stop_times(
    State(app_state): State<AppState>,
    Path((route_id, stop_id)): Path<(String, String)>,
    Query(time): Query<TimeQuery>
) -> Result<Json<StopTimeResponse>, AppError> {
    tracing::debug!("time: {}, date: {}, stop: {}", time.time, time.day, stop_id);
    let stop_name: String = sqlx::query("
        SELECT stop_name from stops WHERE stop_id = ?
        ").bind(&stop_id)
        .fetch_one(&app_state.db).await?.try_get("stop_name")?;

    let route_name: String = sqlx::query("
        SELECT long_name from routes WHERE route_id = ?
        ").bind(&route_id)
        .fetch_one(&app_state.db).await?.try_get("long_name")?;
    
    let rows = sqlx::query("
        SELECT stop_times.arrival_time, trips.headsign, routes.short_name, trips.trip_id FROM stop_times
            INNER JOIN stops ON stops.stop_id = stop_times.stop_id 
            INNER JOIN trips ON trips.trip_id = stop_times.trip_id
            INNER JOIN routes ON routes.route_id = trips.route_id
            WHERE stop_times.stop_id = ? 
            AND trips.route_id = ?
            AND trips.service_id = ?
            ORDER BY stop_times.arrival_time
            ").bind(&stop_id).bind(&route_id).bind(&time.day)
            .fetch_all(&app_state.db)
            .await?;

    let mut results: Vec<StopTime> = Vec::new();
    for row in rows.iter() {
        let arrival_time = Time::from_str(row.try_get("arrival_time")?)?;
        let headsign = row.try_get("headsign")?;
        let trip_id = row.try_get("trip_id")?;
        let route = get_api_route_from_route_id(&route_id)?;
        
        if arrival_time.is_relevant_to(&time.time) {
            let trip_number = get_trip_number_from_id(trip_id)?;
            let mut result = StopTime {
                arrival_time,
                headsign,
                trip_number: trip_number.to_string(),
                actual_platform: Option::None,
                scheduled_platform: Option::None,
                actual_arrival_time: Option::None,
                train_length: Option::None,
            };

            // update next service
            app_state.api.get_next_service(&stop_id, &route, &trip_number).await.map(|line| {
                result.scheduled_platform = Some(line.scheduled_platform.clone());
                if line.computed_departure_time.time != result.arrival_time {
                    result.actual_arrival_time = Some(line.computed_departure_time.time.clone());
                }
                result.actual_platform = Some(line.actual_platform.clone());
            });

            // update service at a glance
            app_state.api.get_service_at_a_glance(trip_number).await.map(|trip| {
                result.train_length = Some(trip.cars);
            });
            
            results.push(result);
        }
    }

    Ok(Json(StopTimeResponse {
        stop_name,
        route_name,
        stop_times: results
    }))
}