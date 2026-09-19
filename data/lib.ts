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

export type Connection = {
    readonly from: number;
    readonly type: "bus" | "underground" | "black" | "taxi";
    readonly to: number;
};

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

export const compareConnections = (a: Connection, b: Connection) =>
    a.from - b.from || compareConnectionType(a.type, b.type) || a.to - b.to;

export const isConnectionInArray = (
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
export const areConnectionsUndirected = (connections: readonly Connection[]) =>
    connections.filter(
        connection =>
            !isConnectionInArray(connections, {
                from: connection.to,
                type: connection.type,
                to: connection.from,
            }),
    );

const formatConnectionTypeForRust = (connectionType: Connection["type"]) => {
    if (connectionType === "black") return "X";
    if (connectionType === "bus") return "B";
    if (connectionType === "taxi") return "T";

    connectionType satisfies "underground";
    return "U";
};

export const formatForRust = (
    connections: readonly Connection[],
) => `// rustfmt::skip
use super::{ConnectionGraph, Connection as C, ConnectionKind::{Taxi as T, Bus as B, Underground as U, Black as X}, Station as S};

pub static CONNECTIONS: ConnectionGraph<${connections.length}> = ConnectionGraph { connections: [${connections.reduce((total, connection) => `${total},\nC {from: S(${connection.from}), kind: ${formatConnectionTypeForRust(connection.type)}, to: S(${connection.to})}`, "").slice(1)}] };`;

const parseAlexElversConnectionType = (
    connectionType: string,
): Connection["type"] => {
    if (connectionType === "water") return "black";
    if (connectionType === "underground") return "underground";
    if (connectionType === "bus") return "bus";
    if (connectionType === "taxi") return "taxi";

    throw new Error("invalid connection type");
};

export const parseConnectionsFromAlexElvers = (connections: string) =>
    connections
        .trim()
        .split(/\r?\n/)
        .map(connection => {
            const [from, to, kind] = connection.split(" ");

            return {
                from: Number(from),
                type: parseAlexElversConnectionType(kind ?? ""),
                to: Number(to),
            } satisfies Connection;
        });

const reverseConnection = (connection: Connection): Connection => ({
    from: connection.to,
    type: connection.type,
    to: connection.from,
});

/**
 * returns a sorted and de-duplicated array of connections
 * containing every connection from the input array plus
 * the reverses of each. the input array must be sorted and
 * deduplicated.
 */
export const addReversedConnections = (
    sortedConnections: readonly Connection[],
) => {
    const allConnections: Connection[] = [
        ...sortedConnections,
        // Add all connections whose reverse is not yet added.
        ...sortedConnections
            .filter(
                connection =>
                    !isConnectionInArray(
                        sortedConnections,
                        reverseConnection(connection),
                    ),
            )
            .map(reverseConnection),
    ];

    allConnections.sort(compareConnections);

    return allConnections;
};
