use k8s_openapi::api::core::v1::Pod;
use kube::{Api, Client};
use kube::runtime::{watcher, WatchStreamExt};
use kube::runtime::watcher::{Config, Event};
use futures::TryStreamExt;
use log::{error, info};
use crate::crash_reporter::crash_message::PodCrashMessage;
use crate::crash_reporter::crash_reporter::CrashReporter;

pub struct PodWatcher{
    api: Api<Pod>,
    crash_reporters: Vec<Box<dyn CrashReporter>>
}

impl PodWatcher{

    /*
        Initialize PodWatcher by providing k8s client and a collection of CrashReporters (allows multiple reporter)
     */
    pub fn new(client: Client, crash_reporters: Vec<Box<dyn CrashReporter>>) -> Self{
        let api = Api::<Pod>::all(client);
        Self {api, crash_reporters}
    }

    /*
        Start watching and handling the k8s events
     */
    pub async fn start_watching(&self) -> Result<(), Box<dyn error::Error>>{
        info!("Started watching pod events");
        let _ = watcher(self.api.clone(), Config::default())
            .default_backoff()
            .try_for_each( |pod_event|  async move {
                match pod_event {
                    Event::Apply(pod) => {
                        self.handle_pod_event(&pod).await
                    }
                    _ => {
                        // Ignore other for now
                    }
                }
                Ok(())
            }).await;
        Ok(())
    }

    /*
        Handle pod event, try to unwrap the message and look if the pod is in CrashLoopBackOff state
     */
    async fn handle_pod_event(&self, pod: &Pod){
        // Try to get the pod name
        let pod_name = pod.metadata.name.clone().unwrap_or_else(|| "Unknown".into());
        // Send out notification in case the pod is in state CrashLoopBackOff
        if let Some(status) = &pod.status{
            if let Some(container_statuses) = &status.container_statuses{
                for container in container_statuses{
                    if let Some(waiting) = &container.state.as_ref().unwrap().waiting{
                        if &waiting.reason.as_ref().unwrap().clone() == "CrashLoopBackOff"{
                            info!("Pod {} is in CrashLoopBackOff", pod_name);
                            self.notify_crash_reporters(
                                PodCrashMessage{
                                    pod_name: pod_name.clone(), state: "CrashLoopBackOff".into()
                                }
                            ).await
                        }
                    }
                }
            }
        }
    }

    /*
        Notify all crash_reporters with the message
     */
    async fn notify_crash_reporters(&self, pod_crash_message: PodCrashMessage){
        self.crash_reporters.iter().for_each(|crash_reporter: &Box<dyn CrashReporter>|{
                crash_reporter.report_crash(&pod_crash_message)
            }
        )
    }
}