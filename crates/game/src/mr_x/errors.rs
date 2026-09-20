use scotland_yard_common::{MrXTicket, Station};

use crate::MrXMove;

#[derive(Debug, thiserror::Error)]
#[error("no {0}")]
pub struct MrXNoTicketError(pub MrXTicket);

#[derive(Debug, thiserror::Error)]
pub enum MrXMoveError {
    #[error("the connection {from} {mov} does not exist")]
    ConnectionDoesNotExist { from: Station, mov: MrXMove },
    #[error("cannot double move {0} because Mr. X has no more double move tickets")]
    CannotDoubleMoveDueToMissingDoubleMoveTicket(MrXMove),
    #[error("Mr. X cannot use the double ticket on two consecutive moves")]
    CannotUseDoubleTicketTwice,
    #[error(
        "cannot move Mr. X {single_move} because detective #{detective_index} is already there"
    )]
    CannotMoveToStationBecauseDetectiveIsThere {
        detective_index: usize,
        single_move: MrXMove,
    },
    #[error("{0}")]
    NoTicket(#[from] MrXNoTicketError),
    #[error("no double move tickets left")]
    NoDoubleMoveTicketLeft,
}
