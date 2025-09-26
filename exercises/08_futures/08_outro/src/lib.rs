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

use std::ops::Deref;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};

pub mod data;
pub mod store;

use axum::{
    routing::{get, post, patch},
    extract::{Path, State},
    http::StatusCode,
    Json, Router
};

use serde::{Deserialize, Serialize};


#[tokio::main]
async fn main() {
    //initialize tracing --> for good diagnostic
    tracing_subscriber::fmt::init();

    let locked_store = Arc::new(RwLock::new(TicketStore::new()));

    //route
    let app = Router::new()
            .route("/get/:id", get(get_handler)) //GET /get path
            .route("/insert", post(insert_handler)) //POST /insert path
            .route("/edit/:id", patch(edit_handler)) //PATCH /edit path
            .with_state(locked_store);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn get_handler(Path(id): Path<TicketId>, State(state): State<Arc<RwLock<TicketStore>>>) -> Json<&Ticket> {
    let store = state.clone();
    let store = store.read().await.deref();
    let ticket = store.get(id).unwrap().read().await.deref();
    Json(ticket)
}