
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use chrono::Local;
use zip::ZipArchive;
use zip::read::ZipFile;
use rusqlite::Connection;
use rusqlite::named_params;
use rustc_hash::FxHashMap;

use crate::parse_mmyy;

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

fn is_train(param: &str) -> bool {
    // busses have number values
    let is_bus = param.parse::<u32>().is_ok();
    !is_bus
}

fn is_train_route(param: u8) -> bool {
    param == 2
}

pub fn gen_db(mut zip: ZipArchive<File>) {
    let db_path = Path::new("app.db");

    // create db
    let mut conn = Connection::open(db_path).expect("failed to create db");
    conn.execute_batch(r"
        PRAGMA journal_mode = OFF;
        PRAGMA synchronous = 0;
        PRAGMA cache_size = 1000000;
        PRAGMA locking_mode = EXCLUSIVE;
        PRAGMA temp_store = MEMORY;"
    ).expect("failed to initialize db");

    create_db_tables(&mut conn).expect("Failed to create db tables");

    let stop_file = zip.by_name("stops.txt").expect("failed to open stops.txt"); 
    populate_stops(&mut conn, stop_file).expect("failed to populate stops table");

    let route_file = zip.by_name("routes.txt").expect("failed to open routes.txt");
    let relevant_str = get_relevant_str(route_file);
    let route_file = zip.by_name("routes.txt").expect("failed to open routes.txt");
    let routes_map = populate_routes(&mut conn, route_file, &relevant_str).expect("failed to populate routes table");

    let trip_file = zip.by_name("trips.txt").expect("failed to open trips.txt");
    let trips_map = populate_trips(&mut conn, trip_file, routes_map).expect("failed to populate trips table");

    let stop_times_file = zip.by_name("stop_times.txt").expect("failed to open stop_times.txt");
    populate_stop_times(&mut conn, stop_times_file, trips_map).expect("failed to populate stop_times table");
}

fn create_db_tables(conn: &mut Connection) -> rusqlite::Result<()>{
    conn.execute_batch(
        r"
        CREATE TABLE routes (
            route_id TEXT PRIMARY KEY,
            short_name TEXT NOT NULL,
            long_name TEXT NOT NULL
        );

        CREATE TABLE trips (
            trip_id TEXT PRIMARY KEY,
            route_id TEXT REFERENCES routes(route_id),
            service_id TEXT,
            short_name TEXT,
            headsign TEXT,
            direction NUMERIC NOT NULL
        );

        CREATE TABLE stops (
            stop_id TEXT PRIMARY KEY,
            stop_name TEXT NOT NULL
        );

        CREATE TABLE stop_times (
            stop_id TEXT REFERENCES stops(stop_id),
            trip_id TEXT REFERENCES trips(trip_id),
            stop_sequence INT NOT NULL,
            arrival_time TEXT,
            departure_time TEXT
        );

        ")
}

fn populate_stops(conn: &mut Connection, file: ZipFile) -> Result<(), rusqlite::Error> {
    let mut reader = BufReader::new(file);
    let mut buf = String::new();

    let tx = conn.transaction()?;
    let mut statement = tx.prepare(r"
        INSERT INTO stops
            VALUES (:stop_id, :stop_name)
    ")?;

    // skip first line
    reader.read_line(&mut buf).unwrap();
    
    loop {
        buf.clear();
        let bytes = reader.read_line(&mut buf).unwrap();
        if bytes == 0 {
            break;
        }

        let params: Vec<&str> = buf.split(',').collect();
        let is_train = is_train(params.get(0).unwrap());
        let is_relevant = params.get(0).unwrap() != &"PA";
        if is_train && is_relevant{
            let stop = Stop {
                stop_id: params.get(0).unwrap().to_string(),
                stop_name: params.get(1).unwrap().to_string(),
            };

            statement.execute(named_params! {
                ":stop_id": stop.stop_id,
                ":stop_name": stop.stop_name,
            })?;
        }
    }
    statement.finalize()?;
    tx.commit()
}

fn populate_routes(conn: &mut Connection, file: ZipFile, relevant_str: &str) -> Result<FxHashMap<String, Route>, rusqlite::Error> {
    let mut reader = BufReader::new(file);
    let mut buf = String::new();

    let tx = conn.transaction()?;

    let mut statement = tx.prepare(r"
        INSERT INTO routes
            VALUES (:route_id, :short_name, :long_name)
    ")?;

    let mut map: FxHashMap<String, Route> = FxHashMap::default();

    // skip first line
    reader.read_line(&mut buf).unwrap();
    loop {
        buf.clear();
        let bytes = reader.read_line(&mut buf).unwrap();
        if bytes == 0 {
            break;
        }

        let params: Vec<&str> = buf.split(',').collect();
        let is_train_route = is_train_route( params.get(4).unwrap().parse::<u8>().unwrap());
        let is_relevant = params.get(0).unwrap().contains(relevant_str);
        if is_train_route && is_relevant {
            let route = Route {
                route_id: params.get(0).unwrap().to_string(),
                short_name: params.get(2).unwrap().to_string(),
                long_name: params.get(3).unwrap().to_string(),
            };
    
            statement.execute(named_params! {
                ":route_id": route.route_id,
                ":short_name": route.short_name,
                ":long_name": route.long_name,
            })?;

            map.insert(route.route_id.clone(), route);
        }
    }
    statement.finalize()?;
    tx.commit()?;
    Ok(map)
    // buf.split(',');
}

fn get_relevant_str(file: ZipFile) -> String {
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    // skip first line
    reader.read_line(&mut buf).unwrap();
    loop {
        buf.clear();
        let bytes = reader.read_line(&mut buf).unwrap();
        if bytes == 0 {
            break;
        }

        let params: Vec<&str> = buf.split(',').collect();
        let is_train_route = is_train_route( params.get(4).unwrap().parse::<u8>().unwrap());
        let date_range_str = params.get(0).unwrap().split("-").collect::<Vec<&str>>()[0];
        if is_train_route {
            let start_date_str = &date_range_str[..4];
            let end_date_str = &date_range_str[4..];
            let start_date = parse_mmyy(start_date_str).unwrap();
            let end_date = parse_mmyy(end_date_str).unwrap();
            let today = Local::now().naive_local().date();
            if today >= start_date && today <= end_date {
                return date_range_str.to_owned()
                // return 
            }
        }

    }
    panic!("No relevant date range found")
}

fn populate_trips(conn: &mut Connection, file: ZipFile, map_routes: FxHashMap<String, Route>) -> Result<FxHashMap<String, Trip>, rusqlite::Error> {
    let mut reader = BufReader::new(file);
    let mut buf = String::new();

    let tx = conn.transaction()?;

    let mut statement = tx.prepare(r"
        INSERT INTO trips
            VALUES (:trip_id, :route_id, :service_id, :short_name, :headsign, :direction);
    ")?;
    let mut map_trips: FxHashMap<String, Trip> = FxHashMap::default();

    // skip first line
    reader.read_line(&mut buf).unwrap();
    loop {
        buf.clear();
        let bytes = reader.read_line(&mut buf).unwrap();
        if bytes == 0 {
            break;
        }

        let params: Vec<&str> = buf.split(',').collect();
        let trip = Trip {
            route_id: params.get(0).unwrap().to_string(),
            service_id: params.get(1).unwrap().to_string(),
            trip_id: params.get(2).unwrap().to_string(),
            short_name: params.get(4).unwrap().to_string(),
            headsign: params.get(3).unwrap().to_string(),
            direction: params.get(5).unwrap().parse::<u8>().unwrap(),
        };

        if map_routes.get(&trip.route_id).is_some() {
            statement.execute(named_params! {
                ":trip_id": trip.trip_id,
                ":route_id": trip.route_id,
                ":service_id": trip.service_id,
                ":short_name": trip.short_name,
                ":headsign": trip.headsign,
                ":direction": trip.direction,
            })?;
            map_trips.insert(trip.trip_id.clone(), trip);
        }
    }
    statement.finalize()?;
    tx.commit()?;
    Ok(map_trips)
    // buf.split(',');
}


fn populate_stop_times(conn: &mut Connection, file: ZipFile, map: FxHashMap<String, Trip>) -> rusqlite::Result<()> {
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    let tx = conn.transaction()?;

    let mut statement = tx.prepare(r"
        INSERT INTO stop_times
            VALUES (:stop_id, :trip_id, :stop_sequence, :arrival_time, :departure_time)
    ")?;

    // skip first line
    reader.read_line(&mut buf).unwrap();
    loop {
        buf.clear();
        let bytes = reader.read_line(&mut buf).unwrap();
        if bytes == 0 {
            break;
        }

        let params: Vec<&str> = buf.split(',').collect();
        let stop_time = StopTime {
            stop_id: params.get(3).unwrap().to_string(),
            trip_id: params.get(0).unwrap().to_string(),
            stop_sequence: params.get(4).unwrap().parse::<u32>().unwrap(),
            arrival_time: params.get(1).unwrap().to_string(),
            departure_time: params.get(2).unwrap().to_string(),
        };

        if map.get(&stop_time.trip_id).is_some() {
            statement.execute(named_params! {
                ":stop_id": stop_time.stop_id,
                ":trip_id": stop_time.trip_id,
                ":stop_sequence": stop_time.stop_sequence,
                ":arrival_time": stop_time.arrival_time,
                ":departure_time": stop_time.departure_time,
            })?;
        }
    }
    statement.finalize()?;
    tx.commit()
}

// fn create_virtual_table(conn: &Connection) -> rusqlite::Result<()> {
//     conn.execute_batch(r"
//         CREATE VIRTUAL TABLE stops_fts
//             USING FTS5(stop_id, stop_name, tokenize='trigram');
//         INSERT INTO stops_fts 
//             SELECT stop_id, stop_name FROM Stops
//     ")?;

//     Ok(())
// }