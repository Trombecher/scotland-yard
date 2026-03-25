const bytes = await Bun.file(".\\standard_correction.safemap").bytes();

let i = 1;

const connections = new Map();

const C_TAXI = "T";
const C_BUS = "B";
const C_METRO = "M";
const C_SHIP = "S";

const collectOfType = (field, ty) => {
    let count = bytes[i];
    i += count;

    while(count--) {
        const otherField = bytes[i - count];

        if(field > otherField) connections.set((field << 8) | otherField, ty);
        else connections.set((otherField << 8) | field, ty);
    }

    i++;
}

for(let field = 1; field < 200; field++) {
    collectOfType(field, C_TAXI);
    collectOfType(field, C_BUS);
    collectOfType(field, C_METRO);
    collectOfType(field, C_SHIP);
}

await Bun.write("data.tsv", connections
    .entries()
    .map(([key, value]) => `${key & 255}\t${value}\t${key >> 8}`)
    .toArray()
    .join("\n"));