use std::time::Instant;

/// Measures the elapsed time of a given function and returns a formatted string representation.
///
/// This function takes a closure as an argument, executes it, and measures the time it takes to run.
/// The elapsed time is then formatted into a human-readable string.
///
/// # Arguments
///
/// * `f` - A closure that takes no arguments and returns nothing (`FnOnce()`).
///
/// # Returns
///
/// A `String` representing the formatted elapsed time.
///
/// # Example
///
/// ```
/// use std::thread::sleep;
/// use std::time::Duration;
/// use dupefiles::elapsed_time::measure_elapsed_time;
///
/// let elapsed_time = measure_elapsed_time(|| {
///     sleep(Duration::from_millis(1500));
/// });
/// assert!(elapsed_time == "1s 500ms" || elapsed_time == "1s 501ms");
/// ```
pub fn measure_elapsed_time<F>(f: F) -> String
where
    F: FnOnce(),
{
    let start = Instant::now();
    f();
    let duration = start.elapsed();
    format_duration(duration)
}

/// Formats a Duration into a human-readable string.
///
/// This function takes a Duration and converts it into a string representation
/// with appropriate units (hours, minutes, seconds, milliseconds) based on the duration's length.
///
/// # Arguments
///
/// * `duration` - A `std::time::Duration` to be formatted.
///
/// # Returns
///
/// A `String` representing the formatted duration.
///
/// # Examples
///
/// ```
/// use std::time::Duration;
/// use dupefiles::elapsed_time::format_duration;
///
/// assert_eq!(format_duration(Duration::from_secs(60*60+60+1)), "1h 1m 1s 0ms");
/// assert_eq!(format_duration(Duration::from_secs(61)), "1m 1s 0ms");
/// assert_eq!(format_duration(Duration::from_secs(1)), "1s 0ms");
/// assert_eq!(format_duration(Duration::from_millis(500)), "500ms");
/// ```
pub fn format_duration(duration: std::time::Duration) -> String {
    let total_seconds = duration.as_secs();
    let hours = total_seconds / 3600;
    let days = hours / 24;
    let weeks = days / 7;
    
    let remaining_days = days % 7;
    let remaining_hours = hours % 24;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    let milliseconds = duration.subsec_millis();

    if weeks > 0 {
        format!("{}w {}d {}h {}m {}s {}ms", weeks, remaining_days, remaining_hours, minutes, seconds, milliseconds)
    } else if days > 0 {
        format!("{}d {}h {}m {}s {}ms", days, remaining_hours, minutes, seconds, milliseconds)
    } else if hours > 0 {
        format!("{}h {}m {}s {}ms", hours, minutes, seconds, milliseconds)
    } else if minutes > 0 {
        format!("{}m {}s {}ms", minutes, seconds, milliseconds)
    } else if seconds > 0 {
        format!("{}s {}ms", seconds, milliseconds)
    } else {
        format!("{}ms", milliseconds)
    }
}