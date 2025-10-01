use crate::store::TicketId;
use serde::Deserialize;
use ticket_fields::{TicketDescription, TicketTitle};

#[derive(Clone, Debug, PartialEq, serde::Serialize)]
pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, serde::Serialize)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}
