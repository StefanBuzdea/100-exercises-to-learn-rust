use std::sync::Arc;
use tokio::sync::RwLock;

use outro_08::store::TicketStore;

// pub mod store;

use axum::{
    routing::{get},
    Router
};

#[tokio::main]
async fn main() {
    //initialize tracing --> for good diagnostic
    tracing_subscriber::fmt::init();

    let locked_store = Arc::new(RwLock::new(TicketStore::new()));

    //route
    let app = Router::new()
            .route("/get/{id}", get(outro_08::get_handler)) //GET /get path
            .route("/insert", get(outro_08::insert_form).post(outro_08::insert_handler)) //POST /insert path
            .route("/edit", get(outro_08::edit_form).post(outro_08::edit_handler)) //PATCH /edit path
            .with_state(locked_store);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // web page available at adress: http://127.0.0.1:3000/

    axum::serve(listener, app).await.unwrap();
}