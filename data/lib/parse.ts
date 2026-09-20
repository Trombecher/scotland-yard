import type {Connection} from ".";

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

let _currentConnections: undefined | Promise<readonly Connection[]>;

export const currentConnections = () =>
    (_currentConnections ??= import("../current/connections.json", {
        with: {type: "json"},
    }).then(x => x.default as readonly Connection[]));
