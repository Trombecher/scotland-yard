import {writeFile} from "node:fs/promises";
import {
    addReversedConnections,
    compareConnections,
    parseConnectionsFromAlexElvers,
} from "./lib";
import unparsedAlexConnections from "./sources/alex connections.txt" with {
    type: "plain",
};

const alexConnections = parseConnectionsFromAlexElvers(unparsedAlexConnections);
alexConnections.sort(compareConnections);

const allAlexConnections = addReversedConnections(alexConnections);

await writeFile("./temp.json", JSON.stringify(allAlexConnections, null, 4));
