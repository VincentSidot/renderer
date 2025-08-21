//! Example demonstrating the renderer logger with and without location display

#[cfg(feature = "logger")]
use log::{debug, error, info, trace, warn};
#[cfg(feature = "logger")]
use renderer::{Level, init_logger};

#[cfg(feature = "logger")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Renderer Logger Demo");
    println!("====================");

    // Initialize logger with location display
    println!("\n1. Logger with location display:");
    if let Err(e) = init_logger(Level::Trace, true) {
        eprintln!("Failed to initialize logger: {}", e);
        return Ok(());
    }

    info!("This is an info message with location");
    warn!("This is a warning message with location");
    error!("This is an error message with location");
    debug!("This is a debug message with location");
    trace!("This is a trace message with location");

    // Demonstrate that logger can't be initialized twice
    println!("\n2. Attempting to re-initialize logger (should fail gracefully):");
    match init_logger(Level::Info, false) {
        Ok(()) => println!("Logger re-initialized successfully"),
        Err(e) => println!("Failed to re-initialize logger (expected): {}", e),
    }

    // Show that we can still log messages
    info!("This message should still appear with location");
    warn!("This warning should also appear with location");

    println!("\nDemo completed successfully!");
    Ok(())
}

#[cfg(not(feature = "logger"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Logger feature is not enabled. Please enable the 'logger' feature to run this demo.");
    Ok(())
}
