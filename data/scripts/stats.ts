import {currentConnections} from "../lib/parse";
import {indexConnectionCount} from "./indexed";

const connectionCountOfStations = indexConnectionCount(
    await currentConnections(),
);

const statisticOverConnectionCount = (
    compare: (a: number, b: number) => boolean,
): [number, number] => {
    return connectionCountOfStations
        .entries()
        .reduce(
            ([minStation, minConnectionCount], [station, connectionCount]) => {
                if (compare(connectionCount, minConnectionCount)) {
                    return [station, connectionCount];
                } else {
                    return [minStation, minConnectionCount];
                }
            },
        );
};

const [stationWithLeastConnections, leastConnections] =
    statisticOverConnectionCount((a, b) => a < b);

const [stationWithMostConnections, mostConnections] =
    statisticOverConnectionCount((a, b) => a > b);

console.log(
    `station with least connections ${stationWithLeastConnections}, count: ${leastConnections}`,
);
console.log(
    `station with most connections ${stationWithMostConnections}, count: ${mostConnections}`,
);
