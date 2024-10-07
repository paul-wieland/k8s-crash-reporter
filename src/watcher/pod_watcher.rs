use k8s_openapi::api::core::v1::Pod;
use kube::{Api, Client};
use kube::runtime::{watcher, WatchStreamExt};
use kube::runtime::watcher::{Config, Event};
use futures::TryStreamExt;
use log::{error, info};

pub struct PodWatcher{
    pods_api: Api<Pod>
}

impl PodWatcher{

    pub fn new(client: Client) -> Self{
        let api = Api::<Pod>::all(client);
        Self {pods_api: api}
    }

    pub async fn start_watching(&self) -> Result<(), Box<dyn std::error::Error>>{
        println!("Started watching pod events");

        let _ = watcher(self.pods_api.clone(), Config::default())
            .default_backoff()
            .try_for_each( |pod_event|  async move {
                match pod_event {
                    Event::Apply(pod) => {
                        Self::handle_pod_event(&pod).await
                    }
                    _ => {
                        // Ignore other for now
                    }
                }
                Ok(())
            }).await;
        Ok(())
    }

    async fn handle_pod_event(pod: &Pod){
        // Try to get the pod name
        let pod_name = pod.metadata.name.clone().unwrap_or_else(|| "Unknown".into());
        // Send out notification in case the pod is in state CrashLoopBackOff
        if let Some(status) = &pod.status{
            if let Some(container_statuses) = &status.container_statuses{
                for container in container_statuses{
                    if let Some(waiting) = &container.state.as_ref().unwrap().waiting{
                        if &waiting.reason.as_ref().unwrap().clone() == "CrashLoopBackOff"{
                            info!("Pod {} is in CrashLoopBackOff", pod_name);
                            error!("Pod {} is in CrashLoopBackOff", pod_name)
                        }
                    }
                }
            }
        }
    }

}