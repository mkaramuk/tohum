use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

/// Creates a spinner progress bar for indeterminate tasks
pub fn create_spinner(message: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.green} {msg}")
            .unwrap()
            .tick_strings(&["∙∙∙", "●∙∙", "∙●∙", "∙∙●", "∙∙∙"]),
    );
    pb.set_message(message.to_string());

    pb
}
