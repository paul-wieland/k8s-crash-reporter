mod watcher;
mod helper;
mod crash_reporter;

use kube::{Client};
use crate::crash_reporter::error_logger_crash_reporter::ErrorLoggerCrashReporter;
use crate::crash_reporter::crash_reporter::CrashReporter;
use crate::helper::logger::setup_logger;
use crate::helper::prints::print_welcome_message;
use crate::watcher::pod_watcher::PodWatcher;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

    // Logger
    setup_logger();

    // Welcome Banner
    print_welcome_message();

    // Setup k8s client
    let client: Client = Client::try_default().await?;

    let mut crash_reporters: Vec<Box<dyn CrashReporter>> = Vec::new();
    crash_reporters.push(Box::new(ErrorLoggerCrashReporter::new()));

    // Initialize pod events watcher
    let pod_watcher = PodWatcher::new(client, crash_reporters);

    // Listening to k8s pod events
    pod_watcher.start_watching().await
}


