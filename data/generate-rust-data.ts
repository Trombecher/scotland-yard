import {access, constants, writeFile} from "node:fs/promises";
import _connections from "./current/connections.json" with {type: "json"};
import {type Connection, formatForRust} from "./lib";

_connections satisfies {from: number; type: string; to: number}[];

const connections = _connections as readonly Connection[];

// this script must be executed in the ./data dir
await access("./package.json", constants.F_OK);

await writeFile(
    "../crates/common/src/connections/data.rs",
    formatForRust(connections),
);
