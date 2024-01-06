export interface Stop {
    stopId: string;
    stopName: string;
}

export interface Route {
    routeId: string;
    routeName: string;
}

export interface StopTimeResponse {
    stopName: string;
    routeName: string;
    stopTimes: StopTime[];
}

export interface StopTime {
    arrivalTime: string;
    headsign: string;
    scheduledPlatform: string;
    actualPlatform: string;
    actualArrivalTime: string;
    trainLength: string;
}
