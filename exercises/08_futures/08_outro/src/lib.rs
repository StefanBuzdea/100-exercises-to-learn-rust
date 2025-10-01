// This is our last exercise. Let's go down a more unstructured path!
// Try writing an **asynchronous REST API** to expose the functionality
// of the ticket management system we built throughout the course.
// It should expose endpoints to:
//  - Create a ticket
//  - Retrieve ticket details
//  - Patch a ticket
//
// Use Rust's package registry, crates.io, to find the dependencies you need
// (if any) to build this system.

use std::sync::Arc;
use tokio::sync::RwLock;

use crate::data::{Status, Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use ticket_fields::{TicketDescription, TicketTitle};


pub mod data;
pub mod store;

use axum::{
    debug_handler,
    extract::{Path, State, Form},
    response::Html,
    http::StatusCode,
    Json
};

use serde::{Deserialize};

#[debug_handler]
pub async fn get_handler(Path(id): Path<u64>, State(state): State<Arc<RwLock<TicketStore>>>) -> Result<Json<Ticket>, StatusCode> {
    let store = state.read().await;
    
    if let Some(ticket_lock) = store.get(TicketId(id)) {
        let ticket = ticket_lock.read().await.clone();
        Ok(Json(ticket))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn insert_form() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/insert" method="post">
                    <label for="title">
                        Enter the ticket title:
                        <input type="text" name="title">
                    </label>

                    <label>
                        Enter the ticket description:
                        <input type="text" name="description">
                    </label>

                    <input type="submit" value="Insert Ticket">
                </form>
            </body>
        </html>
        "#,
    )
}

#[derive(Deserialize, Debug)]
pub struct TicketDraftTemplate {
    title: String,
    description: String
}

#[debug_handler]
pub async fn insert_handler(State(state): State<Arc<RwLock<TicketStore>>>, Form(input): Form<TicketDraftTemplate>) -> Result<Html<String>, StatusCode>
{
    let ticket_draft = TicketDraft {
        title: TicketTitle::try_from(input.title).unwrap(),
        description: TicketDescription::try_from(input.description).unwrap()
    };

    let mut store = state.write().await;

    let new_ticket_id = store.add_ticket(ticket_draft);

    if let Some(ticket_lock) = store.get(new_ticket_id) {
        let new_ticket = ticket_lock.read().await.clone();
        Ok(Html(format!("Succesfully added new ticket with\nticket title = '{:?}'\nticket description = '{:?}'\n", &new_ticket.title, &new_ticket.description)))
    } else {
        Err(StatusCode::EXPECTATION_FAILED)
    }
}



pub async fn edit_form() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/edit" method="post">
                    <label for="id">
                        Enter the ticket id:
                        <input type="number" name="id" required="required">
                    </label>

                    <label for="title">
                        Enter the ticket title:
                        <input type="text" name="title">
                    </label>

                    <label for="description">
                        Enter the ticket description:
                        <input type="text" name="description">
                    </label>

                    <label for="status">
                        Enter the ticket status:
                        <select name="status">
                            <option value="ToDo">ToDo</option>
                            <option value="InProgress">InProgress</option>
                            <option value="Done">Done</option>
                        </select>
                    </label>

                    <input type="submit" value="Edit Ticket">
                </form>
            </body>
        </html>
        "#,
    )
}

#[derive(Deserialize, Debug)]
pub struct TicketPatchTemplate {
    id: u64,
    title: String,
    description: String,
    status: String
}

#[debug_handler]
pub async fn edit_handler(State(state): State<Arc<RwLock<TicketStore>>>, Form(input): Form<TicketPatchTemplate>) -> Result<Html<String>, StatusCode> {
    let searched_ticket_id = TicketId(input.id);

    let store = state.read().await;

    if let Some(ticket_to_be_edited) = store.get(searched_ticket_id) {

        let mut ticket_to_be_edited = ticket_to_be_edited.write().await;

        if input.title.len() != 0 {
            ticket_to_be_edited.title = TicketTitle::try_from(input.title).unwrap();
        }
        if input.description.len() != 0 {
            ticket_to_be_edited.description = TicketDescription::try_from(input.description).unwrap();
        }
        if input.status.len() != 0 {
            match input.status.as_str() {
                "ToDo" => ticket_to_be_edited.status = Status::ToDo,
                "InProgress" => ticket_to_be_edited.status = Status::InProgress,
                "Done" => ticket_to_be_edited.status = Status::Done,
                _ => ticket_to_be_edited.status = ticket_to_be_edited.status
            };
        }
        Ok(Html(format!("Succesfully edited ticket with id {}", &input.id)))

    } else {
        Err(StatusCode::NOT_FOUND)
    }
}