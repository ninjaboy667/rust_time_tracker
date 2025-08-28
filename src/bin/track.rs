use rust_time_tracker::{error::AppError, feature::cli, init};
use error_stack::{Result, ResultExt};

// track is the binary anme
// track 
// track start
// track stop
// track report
fn main () -> Result<(), AppError> {
    // Initialize error reporting lets us get prettty errors with suggestions
    init::error_reporting();
    init::tracing();

    cli::run()
        .change_context(AppError)
        .attach_printable("failed to run CLI")?;

    Ok(())
}