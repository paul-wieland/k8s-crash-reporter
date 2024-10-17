mod watcher;
mod helper;
mod crash_reporter;

use teloxide::prelude::*;
use crate::crash_reporter::error_logger_crash_reporter::ErrorLoggerCrashReporter;
use crate::crash_reporter::crash_reporter::CrashReporter;
use crate::crash_reporter::telegram_crash_reporter::TelegramCrashReporter;
use crate::helper::logger::setup_logger;
use crate::helper::prints::print_welcome_message;
use crate::watcher::kubernetes_pod_watcher::KubernetesPodWatcher;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

    // Logger
    setup_logger();

    // Welcome Banner
    print_welcome_message();

    let mut crash_reporters: Vec<Box<dyn CrashReporter>> = Vec::new();
    crash_reporters.push(Box::new(ErrorLoggerCrashReporter::new()));

    let telegram_crash_reporter = TelegramCrashReporter::new()?;
    crash_reporters.push(Box::new(telegram_crash_reporter));


    // Initialize kubernetes pod events watcher
    let pod_watcher = KubernetesPodWatcher::new(crash_reporters).await?;

    // Listening to k8s pod events
    pod_watcher.start_watching().await
}


