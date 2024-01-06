import { getStopTimes } from '$lib/api';
import { dateToApiDate, timeToApiTime } from '$lib/utils';
import type { PageLoad } from './$types';
export const load = (async ({ fetch, params }) => {
    let date = new Date();
    let currentTime = date.toLocaleTimeString("en-US", { timeZone: "America/Toronto"});
    let currentDate = date.toLocaleDateString();
    let stopTimes = await getStopTimes(fetch, params.stopId, params.routeId, dateToApiDate(currentDate), timeToApiTime(currentTime));
    return stopTimes;

    // let test = {
    //     stopName: "test",
    //     routeName: "test",
    //     stopTimes: [
    //     {
    //         "arrivalTime": "20:42:0",
    //         "headsign": "LE - Union Station",
    //         "tripNumber": "9235",
    //         "scheduledPlatform": "5,6",
    //         "actualPlatform": undefined,
    //         "actualArrivalTime": undefined,
    //         "trainLength": "12"
    //     },
    //     {
    //         "arrivalTime": "20:42:0",
    //         "headsign": "LE - Union Station",
    //         "tripNumber": "9235",
    //         "scheduledPlatform": "5,6",
    //         "actualPlatform": "7",
    //         "actualArrivalTime": undefined,
    //         "trainLength": "12"
    //     },
    //         {
    //             "arrivalTime": "20:42:0",
    //             "headsign": "LE - Union Station",
    //             "tripNumber": "9235",
    //             "scheduledPlatform": "5,6",
    //             "actualPlatform": "7",
    //             "actualArrivalTime": "20:43:0",
    //             "trainLength": "12"
    //             },
    //     ]
    // }

    // return test;
}) satisfies PageLoad;