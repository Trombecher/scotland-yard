const dataString = await Bun.file("data.tsv").text();

const connections = dataString.split("\n").reduce((connections, line) => {
    let [from, ty, to] = line.split("\t");
    connections.push([+from, ty, +to]);
    return connections;
}, []);

const out = connections.reduce((out, [from, ty, to]) => out + `(Field::new(${from}).unwrap(), ${ty}, Field::new(${to}).unwrap()),\n`, "")

await Bun.write("connections.rs", out);