use std::env;
use std::error::Error;
use async_trait::async_trait;
use log::{error, info};
use teloxide::Bot;
use crate::crash_reporter::crash_message::PodCrashMessage;
use crate::crash_reporter::crash_reporter::CrashReporter;
use teloxide::prelude::*;

pub struct TelegramCrashReporter{
    bot: Bot,
    chat_id: String
}

impl TelegramCrashReporter{

    pub fn new() -> Result<Self, Box<dyn Error>>{
        let token = Self::get_env_variable("TOKEN".into())?;
        let chat_id = Self::get_env_variable("CHAT_ID".into())?;
        Ok(Self { bot: Bot::new(token), chat_id })
    }

    fn get_env_variable(env_variable_name: String) -> Result<String, Box<dyn Error>>{
        return match env::var(&env_variable_name){
            Ok(token) => Ok(token),
            Err(_) => Err(format!("Environment variable {} is missing", env_variable_name).into())
        }
    }

}

#[async_trait]
impl CrashReporter for TelegramCrashReporter{
    async fn report_crash(&self, pod_crash_message: &PodCrashMessage) {
        if let Ok(_) = self.bot.send_message(self.chat_id.clone(), pod_crash_message.formatted_message()).await{
            info!("Published message to telegram: {}", pod_crash_message.formatted_message())
        }else{
            error!("Could not publish message to telegram: {}", pod_crash_message.formatted_message())
        }
    }
}