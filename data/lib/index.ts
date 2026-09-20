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
