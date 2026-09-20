import type {Connection} from ".";

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
use super::{ConnectionGraph, Connection as C, ConnectionKind::{Taxi as T, Bus as B, Underground as U, Black as X}};
use crate::Station as S;

pub static CONNECTIONS: ConnectionGraph<${connections.length}> = ConnectionGraph { connections: [${connections.reduce((total, connection) => `${total},\nC {from: S::new(${connection.from}).unwrap(), kind: ${formatConnectionTypeForRust(connection.type)}, to: S::new(${connection.to}).unwrap()}`, "").slice(1)}] };`;
