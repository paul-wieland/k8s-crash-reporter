

pub struct PodCrashMessage{
    pod_name: String,
    state: String
}

impl PodCrashMessage{


    pub fn new(pod_name: String, state: String) -> Self{
        Self{pod_name, state}
    }

    pub fn formatted_message(&self) -> String{
        format!("{}: {}", self.pod_name.clone(), self.state.clone())
    }

}