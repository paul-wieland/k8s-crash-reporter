use async_trait::async_trait;
use crate::crash_reporter::crash_message::PodCrashMessage;

#[async_trait]
pub trait CrashReporter{

    async fn report_crash(&self, pod_crash_message: &PodCrashMessage);

}