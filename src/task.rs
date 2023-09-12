use serde::Serialize;
use std::fmt::{Debug, Error, Formatter};
use std::sync::Arc;
use strum_macros::{Display, EnumString, EnumVariantNames};
use tokio::time::{Duration, Instant};

use crate::error::CWError;

#[derive(Serialize, Display, Copy, Clone, PartialEq, EnumString, EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub(crate) enum Status {
    CountingDown,
    ActionSuccessful,
    ActionFailed,
}

impl Debug for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "Status: {}", self)
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Task {
    // Status
    pub status: Status,
    pub runs_at: Instant,
    // Action
    pub command: Arc<dyn Command + Send + Sync>,
    pub result: String,
}

impl Task {
    pub(crate) fn new(command: Arc<dyn Command + Send + Sync>, duration: Duration) -> Task {
        Task {
            // Status
            status: Status::CountingDown,
            runs_at: Instant::now() + duration,
            // Action
            command,
            result: "pending execution".to_string(),
        }
    }

    pub(crate) fn expires_in(self) -> u64 {
        let duration = self.runs_at.saturating_duration_since(Instant::now());
        duration.as_secs()
    }

    pub(crate) fn reset(&mut self, duration: Duration) -> Result<(), CWError> {
        if self.status != Status::CountingDown {
            return Err(CWError::new(
                "Too late",
                "You're too late, this clock has already reached zero. To re-arm, please restart the application.",
            ));
        }
        self.runs_at = Instant::now() + duration;
        Ok(())
    }

    pub(crate) fn run(&mut self) {
        match self.command.run() {
            Err(e) => {
                self.result = e;
                self.status = Status::ActionFailed;
            }
            Ok(r) => {
                self.result = r;
                self.status = Status::ActionSuccessful;
            }
        }

        println!("run({}): {}", self.command.name(), self.result)
    }
}

pub(crate) trait Command {
    fn run(&self) -> Result<String, String>;
    fn name(&self) -> &'static str;
}

impl Debug for dyn Command + Sync + Send {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "Command {{ Name: {} }}", self.name())
    }
}
