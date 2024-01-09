struct Stop {
    stop_id: String,
    stop_name: String,
}

struct Route {
    route_id: String,
    short_name: String,
    long_name: String,
}

struct Trip {
    trip_id: String,
    route_id: String,
    service_id: String,
    short_name: String,
    headsign: String,
    direction: u8
}

struct StopTime {
    stop_id: String,
    trip_id: String,
    stop_sequence: u32,
    arrival_time: String,
    departure_time: String,
}