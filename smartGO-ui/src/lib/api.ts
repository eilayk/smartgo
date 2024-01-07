import type { Route, Stop, StopTimeResponse } from "./models";

const PUBLIC_API_URL = import.meta.env.VITE_API_URL;

export const getStops = async (fetch: (url: string) => Promise<Response> ): Promise<Stop[]> => {
    try {
        let resp = await fetch (`${PUBLIC_API_URL}/api/stops`);
        const stops: Stop[] = await resp.json();
        return stops;
    } catch (err) {
        console.error(err);
        return [];
    }
}

export const getRoutesPerStop = async (fetch: (url: string) => Promise<Response>, stopId: string): Promise<Route[]> => {
    try {
        let resp = await fetch (`${PUBLIC_API_URL}/api/route/stop/${stopId}`);
        const routes: Route[] = await resp.json();
        return routes;
    } catch (err) {
        console.error(err);
        return [];
    }
}

export const getStopTimes = async (fetch: (url: string) => Promise<Response>, stopId: string, routeId: string, day: string, time: string): Promise<StopTimeResponse> => {
    try {
        let resp = await fetch (`${PUBLIC_API_URL}/api/route/${routeId}/stop/${stopId}?day=${day}&time=${time}`);
        const stopTimes: StopTimeResponse = await resp.json();
        return stopTimes;
    } catch (err) {
        console.error(err);
        throw Error("Error fetching stop times");
    }
}

export const getAllRoutes = async (fetch: (url: string) => Promise<Response>): Promise<Route[]> => {
    try {
        let resp = await fetch (`${PUBLIC_API_URL}/api/routes`);
        const routes: Route[] = await resp.json();
        return routes;
    } catch (err) {
        console.error(err);
        throw Error("Error fetching stop times");
    }
}

export const getStopsPerRoute = async (fetch: (url: string) => Promise<Response>, routeId: string): Promise<Stop[]> => {
    try {
        let resp = await fetch (`${PUBLIC_API_URL}/api/route/${routeId}`);
        const stops: Stop[] = await resp.json();
        return stops;
    } catch (err) {
        console.error(err);
        throw Error("Error fetching stop times");
    }
}

