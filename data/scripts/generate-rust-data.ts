import {access, constants, writeFile} from "node:fs/promises";
import {formatForRust} from "../lib/format";
import {currentConnections} from "../lib/parse";

const connections = await currentConnections();

// this script must be executed in the ./data dir
await access("./package.json", constants.F_OK);

await writeFile(
    "../crates/common/src/connections/data.rs",
    formatForRust(connections),
);
