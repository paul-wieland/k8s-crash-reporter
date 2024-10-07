use crate::crash_reporter::crash_message::PodCrashMessage;

pub trait CrashReporter{

    fn report_crash(&self, pod_crash_message: &PodCrashMessage);

}