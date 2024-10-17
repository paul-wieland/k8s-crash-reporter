use async_trait::async_trait;
use log::error;
use crate::crash_reporter::crash_message::PodCrashMessage;
use crate::crash_reporter::crash_reporter::CrashReporter;

/*
    This implementation is mainly used for testing purpose
 */
pub struct ErrorLoggerCrashReporter {}

impl ErrorLoggerCrashReporter {

    pub fn new() -> Self{
        Self{}
    }
}

#[async_trait]
impl CrashReporter for ErrorLoggerCrashReporter {

    async fn report_crash(&self, pod_crash_message: &PodCrashMessage) {
        error!("{}", pod_crash_message.formatted_message())
    }
}