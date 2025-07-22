#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{Router, routing::get};
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use order_stream::app::{shell, App};
    use order_stream::backend::db;
    use order_stream::backend::websocket::websocket_handler;
    use tower_sessions::{SessionManagerLayer, cookie::SameSite};
    use order_stream::backend::auth::SurrealSessionStore;
    use tokio::sync::broadcast;

    // Initialize database connection
    if let Err(e) = db::initialize_database().await {
        eprintln!("Failed to initialize database: {}", e);
        std::process::exit(1);
    }
    println!("Database initialized successfully");

    // Setting this to None means we'll be using cargo-leptos and its env vars
    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // Create WebSocket broadcast channel
    let (ws_sender, _) = broadcast::channel::<String>(1000);
    
    // Initialize global WebSocket sender for server functions
    order_stream::backend::websocket::init_websocket_sender(ws_sender.clone());
    
    // Configure sessions
    let session_store = SurrealSessionStore::new();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_same_site(SameSite::Lax)
        .with_http_only(true);

    // Create a WebSocket router that includes the WebSocket state
    let ws_router = Router::new()
        .route("/ws", get(websocket_handler))
        .with_state(ws_sender);

    // build our application with routes  
    let app = Router::new()
        .merge(ws_router)
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .layer(session_layer)
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    // run our app with hyper
    // Server functions are automatically registered by Leptos
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("listening on http://{}", &addr);
    println!("Server functions available at /api/*");
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
