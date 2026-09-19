import _connections from "./connections.json" with {type: "json"};

const connections = _connections as readonly Connection[];

type Connection = {
    readonly from: number;
    readonly type: "bus" | "underground" | "black" | "taxi";
    readonly to: number;
};

/*
const connectionsFromStations = () => {
    const connections: Connection[] = [];

    for (const station of stations) {
        const options = [
            [station.neighbourTaxis, "taxi"],
            [station.neighbourBuses, "bus"],
            [station.neighbourUndergrounds, "underground"],
        ] as const satisfies [number[], Connection["type"]][];

        for (const [neighbors, t] of options) {
            for (const neighbourTaxi of neighbors) {
                connections.push({
                    from: station.number,
                    type: t,
                    to: neighbourTaxi,
                });
            }
        }
    }

    // Missing black connections:
    connections.push(
        {from: 108, type: "black", to: 115},
        {from: 115, type: "black", to: 108},
        {from: 115, type: "black", to: 157},
        {from: 157, type: "black", to: 115},
        {from: 157, type: "black", to: 194},
        {from: 194, type: "black", to: 157},
    );

    connections.sort(compareConnections);

    return connections;
};
 */

const compareConnectionType = (
    a: Connection["type"],
    b: Connection["type"],
) => {
    if (a === b) return 0;
    if (
        (a === "taxi" && b !== "taxi") ||
        (a === "bus" && (b === "underground" || b === "black")) ||
        (a === "underground" && b === "black")
    )
        return -1;

    return 1;
};

const compareConnections = (a: Connection, b: Connection) =>
    a.from - b.from || compareConnectionType(a.type, b.type) || a.to - b.to;

const isConnectionInArray = (
    arr: readonly Connection[],
    target: Connection,
): boolean => {
    let low = 0;
    let high = arr.length - 1;

    while (low <= high) {
        const mid = low + ((high - low) >> 1);
        // biome-ignore lint/style/noNonNullAssertion: this exists!
        const cmp = compareConnections(arr[mid]!, target);

        if (cmp === 0) return true;

        // If arr[mid] < target, search upper half; otherwise lower half.
        if (cmp < 0) low = mid + 1;
        else high = mid - 1;
    }

    return false;
};

/**
 * connections must be sorted
 */
const areConnectionsUndirected = (connections: readonly Connection[]) =>
    connections.filter(
        connection =>
            !isConnectionInArray(connections, {
                from: connection.to,
                type: connection.type,
                to: connection.from,
            }),
    );

console.log(areConnectionsUndirected(connections));

// const formatForRust = (connections: Connection[]) => `[${}]`;
