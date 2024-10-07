use crate::crash_reporter::crash_message::PodCrashMessage;
use crate::crash_reporter::crash_reporter::CrashReporter;

pub struct CommandLineCrashReporter{

}

impl CommandLineCrashReporter{

    pub fn new() -> Self{
        Self{}
    }

}

impl CrashReporter for CommandLineCrashReporter{

    fn report_crash(&self, pod_crash_message: &PodCrashMessage) {
        println!("Command Line Crash Reporter: Pod {} is in state {}", pod_crash_message.pod_name, pod_crash_message.state)
    }
}