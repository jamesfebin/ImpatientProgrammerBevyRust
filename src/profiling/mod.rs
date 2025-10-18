//! Profiling utilities module
//! Basic performance monitoring and logging

/// Initialize basic profiling and logging
pub fn init_profiling() {
    // Initialize tracing subscriber for better logging
    tracing_subscriber::fmt::init();
    tracing::info!("📊 Performance monitoring enabled - Watch console for FPS logs");
}