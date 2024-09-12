# smartGO-api
This is a Rust web API that combines [Metrolinx static GTFS data](https://www.metrolinx.com/en/about-us/open-data) with [Metrolinx's GO API](https://api.openmetrolinx.com/OpenDataAPI/Help/Index/en) to show train routes with up-to-date departure times, platform, and train-length.

## How it works
An sqlite database is generated from static GTFS data using [this script](https://github.com/eilayk/smartgo/tree/main/gtfs-to-sqlite).

When querying for stop times, the API:
1. Queries the sqlite database for all trips of the selected route on the selected date
2. Enriches the stop times with data from Metrolinx's GO API

## Endpoints
/api/stops - fetch all stops

/api/routes - fetch all routes

/api/route/stop/$STOP_ID - fetch all routes that serve a particular stop

/api/route/$ROUTE_ID - fetch stops in order for a particular route

/api/route/$ROUTE_ID/stop/$STOP_ID - fetch stop times for a particular route and stop. This endpoint also expects the following path variables:
- day: date in format yyyy/mm/dd
- time: time in 24hr time format

## Running Locally
To run the API locally, an API key needs to be obtained from Metrolinx. It can be supplied as GO_API_KEY environment variable.

1. clone the repository
```bash
git clone https://github.com/eilayk/smartgo/
```

2. build
```bash
cd smartgo/smartgo-api
docker build -t smartgo-api .
```

3. run
```bash
docker run smartgo-api
```
