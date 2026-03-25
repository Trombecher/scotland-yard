const dataString = await Bun.file("data.tsv").text();

const data = [];

dataString.split("\n").forEach(line => {
    let [from, ty, to] = line.split("\t");

    from = +from - 1;
    to = +to - 1;

    const outgoing = data[from];
    if(outgoing) {
        outgoing.push([ty, to]);
    } else {
        data[from] = [[ty, to]];
    }

    const outgoing2 = data[to];
    if(outgoing2) {
        outgoing2.push([ty, from]);
    } else {
        data[to] = [[ty, from]];
    }
});

const out = `use crate::board::Field;
use crate::board::ConnectionType;
use ConnectionType::Taxi as T;
use ConnectionType::Bus as B;
use ConnectionType::Metro as M;
use ConnectionType::Ship as S;

pub static TYPES_TABLE: [[Option<ConnectionType>; 199]; 199] = [${
    data.map(outgoing => {
        return `[${
            Array.from({length: 199}, i => {
                const conn = outgoing.find(([_, to]) => to === i);
                if(conn?.[0]) return `Some(${conn[0]})`;
                else return `None`;
            }).join(",")
        }]`;
    }).join(",\n")
}];

pub static OPTIONS_TABLE: [&[(ConnectionType, Field)]; 199] = [${
    data.map(outgoing => {
        return `&[${
            outgoing.map(([ty, to]) => `(${ty}, unsafe { Field::new_unchecked(${to + 1}) })`).join(",")
        }]`;
    }).join(",\n")
}];`;

await Bun.write("./src/connections.rs", out);

await Bun.$`rustfmt ./src/connections.rs`;