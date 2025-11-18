// web-service/src/main.rs
// Entry point for the HTTP web service that serves HTMX UI

use tracing::{info, Level};
use tracing_subscriber;
use warp::Filter;

// The #[tokio::main] macro sets up the async runtime
// Same as gRPC service, but now we are handling HTTP instead
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .with_max_level(Level::INFO)
        .init();

    info!("🌐 Web Service starting...");
    info!("📍 Version: {}", env!("CARGO_PKG_VERSION"));

    // Define the server address
    // 0.0.0.0 means listen on all network interfaces
    // [u8; 4] is an array of 4 bytes - Rust's way of representing IPv4
    let address: [u8; 4] = [0, 0, 0, 0];
    let port: u16 = 3000;

    info!("🎯 Server will listen on http://{}:{}", "0.0.0.0", port);

    // Create a simple health check route
    // This demonstrates Warp's filter-based routing
    // Filters are composable building blocks
    let health_route = warp::path("health")
        // Match GET requests
        .and(warp::get())
        // When matched, respond with JSON
        .map(|| {
            // warp::reply::json creates a JSON response
            // This struct will be serialized automatically
            warp::reply::json(&serde_json::json!({
                "status": "healthy",
                "service": "web-service",
                "version": env!("CARGO_PKG_VERSION")
            }))
        });

    // Create a root route that shows a welcome message
    let root_route = warp::path::end().and(warp::get()).map(|| {
        // Return HTML directly
        // In phase 3, we'll use Askama templates instead
        warp::reply::html(
            r#"
                <!DOCTYPE html>
                <html>
                <head>
                    <title>Task Manager</title>
                    <meta charset="utf-8">
                </head>
                <body>
                    <h1>🚀 Rust Task Manager</h1>
                    <p>Web service is running!</p>
                    <p>⚠️ UI not yet implemented (Phase 3)</p>
                    <h2>Next Steps:</h2>
                    <ul>
                        <li>Create Askama templates</li>
                        <li>Set up gRPC client to communicate with backend</li>
                        <li>Build HTMX-powered task management UI</li>
                        <li>Implement authentication (Phase 5)</li>
                    </ul>
                    <p><a href="/health">Health Check</a></p>
                </body>
                </html>
                "#,
        )
    });

    // Combine routes using .or()
    // Warp tries each route in order until one matches
    // Python equivalent: @app.route() decorators
    // Rust advantage: routes are type-checked at compile time
    let routes = root_route
        .or(health_route)
        // Add CORS headers for development (will refine in Phase 3)
        .with(warp::cors().allow_any_origin());

    info!("✅ Routes configured:");
    info!("   GET  /        - Welcome page");
    info!("   GET  /health  - Health check endpoint");
    info!("");
    info!("🚀 Server starting on http://localhost:{}", port);
    info!("   Press Ctrl+C to stop");

    // Create the server
    // warp::serve() takes our routes and creates a server
    // .run() starts the server and runs forever (until shutdown)
    let server = warp::serve(routes).run((address, port));

    // In production, we'd add graceful shutdown here
    // For now, just run until Ctrl+C
    server.await;

    info!("👋 Web service stopped");
    Ok(())
}
