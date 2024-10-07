mod watcher;

use kube::{Client};
use crate::watcher::pod_watcher::PodWatcher;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    print_welcome_message();

    env_logger::init();

    let client: Client = Client::try_default().await?;

    let pod_watcher = PodWatcher::new(client);
    pod_watcher.start_watching().await
}

fn print_welcome_message(){
    println!("****************************");
    println!("** Welcome to Pod Watcher **");
    println!("****************************");
}

