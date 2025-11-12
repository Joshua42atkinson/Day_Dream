use axum::Router;


// --- (IMPROVEMENT) Import Leptos routes ---
// These helpers will allow us to host server functions
use leptos::{get_configuration, logging};
use leptos_axum::{generate_route_list, LeptosRoutes};

// --- (IMPROVEMENT) Import frontend components ---
// This allows the backend to know about the server functions
// defined in the frontend crate.
use frontend::app::App; 

mod handlers;
mod routes;
mod domain;

use routes::player::player_routes;

// --- Main Server Function ---
#[tokio::main]
async fn main() {
    logging::log!("Starting Daydream Backend Server...");

    // --- (IMPROVEMENT) Load Leptos Config ---
    // This loads the `frontend/Cargo.toml` (which contains a [package.metadata.leptos] section)
    // to configure server-side rendering, etc.
    let conf = get_configuration(Some("frontend/Cargo.toml")).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App); // Get all routes from the <App/> component

    // --- (IMPROVEMENT) CORS Layer ---
    // This allows requests from our frontend (running on :3000)
    // to our backend (running on :3001)
    use tower_http::cors::{CorsLayer, Any};
    let cors = CorsLayer::new()
        .allow_origin(Any) // Allow any origin for development
        .allow_methods(Any)
        .allow_headers(Any);

    // Build our application router
    let app = Router::new()
        // --- (IMPROVEMENT) Leptos Routes ---
        // This line adds all the routes needed for Leptos server functions.
        // It's separate from the file serving, which `cargo-leptos` handles.
        .leptos_routes(&leptos_options, routes, App)
        .merge(player_routes(&leptos_options))
        .layer(cors) // Apply the CORS middleware
        .with_state(leptos_options);

    // Run the server
    logging::log!("Backend listening on http://{}", &addr);
    axum::serve(tokio::net::TcpListener::bind(&addr).await.unwrap(), app)
        .await
        .unwrap();
}
