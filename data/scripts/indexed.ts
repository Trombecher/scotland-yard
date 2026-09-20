import type {Connection} from "../lib";

export const indexConnectionCount = (connections: readonly Connection[]) => {
    const connectionCountOfStations = new Map<number, number>();

    for (const connection of connections) {
        const newConnectionCount =
            1 + (connectionCountOfStations.get(connection.from) ?? 0);

        connectionCountOfStations.set(connection.from, newConnectionCount);
    }

    return connectionCountOfStations;
};
