import type { AutocompleteOption } from "@skeletonlabs/skeleton"
import type { Stop, StopTime } from "./models"

export const stopToAutocompleteOpton = (stop: Stop): AutocompleteOption => {
    let autoComplete: AutocompleteOption = {
        label: stop.stopName,
        value: stop,
    }
    return autoComplete;
}

export const dateToApiDate = (date: string): string => {
    let [month, day, year] = date.split('/');
    if (month.length === 1) {
        month = `0${month}`;
    }
    if (day.length === 1) {
        day = `0${day}`;
    }
    return `${year}${month}${day}`;
}

export const timeToApiTime = (time: string): string => to24Hr(time).split(' ')[0];

const to24Hr = (time: string): string => {
    let [hr, min, sec] = time.split(':');
    let suffix = time.split(' ')[1];
    if (suffix === 'PM') {
        hr = (parseInt(hr) + 12).toString();
    }
    return `${hr}:${min}:${sec}`;
}

export const nullToEmptyString = (str: string | null): string => {
    if (str === null) {
        return "";
    }
    return str;
}

export const apiTimeToLocalTime = (time: string): string => {
    let [hr, min, sec] = time.split(':');
    let suffix = 'AM';
    let parsedHr = parseInt(hr);
    if (parsedHr > 24) {
        hr = (parsedHr - 24).toString();
    } else if (parsedHr == 24) {
        hr = '12';
    } else if (parsedHr == 12) {
        suffix = 'PM';
    } else if (parsedHr > 12) {
        hr = (parsedHr - 12).toString();
        suffix = 'PM';
    }
    return `${hr}:${min} ${suffix}`;
}

export const stopTimeHasActualPlatform = (stopTime: StopTime): boolean => {
    return stopTime.actualPlatform !== null;
}

export const stopTimeHasActualArrivalTime = (stopTime: StopTime): boolean => {  
    return stopTime.actualArrivalTime !== null;
}