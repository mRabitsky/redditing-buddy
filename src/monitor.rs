use std::error::Error;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use clokwerk::{ScheduleHandle, Scheduler, TimeUnits};

use crate::config::Config;
use crate::discord::DiscordMessenger;
use crate::reddit::Redditor;

pub struct Monitor {
    config: Arc<Mutex<Config>>,
    interval: Duration,
    handle: Option<ScheduleHandle>,

} impl Monitor {
    pub fn new(config: Config, duration: Duration) -> Monitor {
        Monitor {
            config: Arc::new(Mutex::new(config)),
            interval: duration,
            handle: None,
        }
    }
    pub fn start(&mut self) -> Result<(), Box<dyn Error>> {
        let mut scheduler = Scheduler::new();
        let d = DiscordMessenger::new(self.config.lock().unwrap().discord_bot_token.clone())?;
        let mut r = Redditor::new(Arc::clone(&self.config))?;

        scheduler.every((self.interval.as_secs() as u32).seconds()).run(move || {
            let new_posts = r.check();

            if let Err(e) = d.send_all(new_posts) {
                eprintln!("{:?}", e);
            }
        });
        self.handle = Some(scheduler.watch_thread(self.interval));

        Ok(())
    }
    pub fn stop(self) -> Result<(), Box<dyn Error>> {
        if let Some(h) = self.handle {
            h.stop();

            match self.config.lock() {
                Ok(config) => config.write(&config.path),
                Err(_) => Err(From::from("Could not get a lock for the config; this is likely because the scheduling thread panicked and poisoned the guard."))
            }
        } else {
            Err(From::from("This Monitor has already been stopped."))
        }
    }
}
