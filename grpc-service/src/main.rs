// grpc-service/src/main.rs
// Entry point for the gRPC backend service

// These are like Python's imports, but checked at compile time
use tracing::{info, Level};
use tracing_subscriber;

// The #[tokio::main] macro transforms our async main into a regular main
// It sets up the Tokio async runtime for us
// Python equivalent: asyncio.run() but happens automatically
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize structured logging
    // tracing_subscriber sets up log formatting and filtering
    // Like Python's logging.basicConfig() but more powerful
    tracing_subscriber::fmt()
        .with_target(false) // Don't show module paths in logs
        .with_level(true) // Show log level (INFO, WARN, etc.)
        .with_max_level(Level::INFO) // Only show INFO and above
        .init();

    // Log startup message
    // info! is a macro (ends with !) that logs at INFO level
    // Unlike Python's logging.info(), this is zero-cost if disabled
    info!("gRPC Service starting...");
    info!("Version: {}", env!("CARGO_PKG_VERSION"));

    // In Phase 2, we'll:
    // 1. Load configuration from environment variables
    // 2. Set up database connection pool
    // 3. Start gRPC server on port 50051
    // 4. Handle graceful shutdown on SIGTERM/SIGINT
    info!("⚠️  gRPC server not yet implemented (Phase 2)");
    info!("🎯 Next steps:");
    info!("   - Define .proto files for task service");
    info!("   - Generate Rust code with tonic-build");
    info!("   - Implement TaskService trait");
    info!("   - Start server on 0.0.0.0:50051");

    // Simulate a running service for now
    // tokio::time::sleep is like asyncio.sleep() in Python
    info!("💤 Service running in placeholder mode (press Ctrl+C to stop)");

    // Create a channel to wait for Ctrl+C signal
    // This is async-safe signal handling
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for Ctrl+C");

    info!("🛑 Received shutdown signal, cleaning up...");
    info!("👋 gRPC service stopped gracefully");

    // Result<T, E> is Rust's way of handling errors
    // Ok(()) means success with no return value
    // Python equivalent: return None or just return
    Ok(())
}
