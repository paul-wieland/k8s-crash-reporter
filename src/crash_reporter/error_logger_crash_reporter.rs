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

impl CrashReporter for ErrorLoggerCrashReporter {

    fn report_crash(&self, pod_crash_message: &PodCrashMessage) {
        error!("Pod {} is in state {}", pod_crash_message.pod_name, pod_crash_message.state)
    }
}