use crate::error::Error;
use crate::http::User;
use crate::task::{Command, Task};

use std::collections::{BTreeSet, HashMap};
use std::sync::{Arc, Mutex};
use tokio::sync::Notify;
use tokio::time::{self, Duration, Instant};

#[derive(Clone)]
pub(crate) struct MemDB {
    shared: Arc<Shared>,
}

impl MemDB {
    pub(crate) fn new(username: String, password: String) -> MemDB {
        let shared = Arc::new(Shared {
            tasks: Mutex::new(Tasks {
                entries: HashMap::new(),
                expirations: BTreeSet::default(),
            }),
            user: User { username, password },
            notify_task_runner: Notify::new(),
        });

        // Start the background task.
        tokio::spawn(run_expired_tasks(shared.clone()));

        MemDB { shared }
    }

    pub(crate) fn list_tasks(&self) -> Vec<String> {
        // Acquire the lock, get the entry and clone the value.
        let state = self.shared.tasks.lock().unwrap();
        Vec::from_iter(state.entries.keys().cloned())
    }

    pub(crate) fn get_task(&self, name: &str) -> Result<Task, Error> {
        // Acquire the lock, get the entry and clone the value.
        let state = self.shared.tasks.lock().unwrap();
        match state.entries.get(name).cloned() {
            Some(task) => Ok(task),
            None => Err(Error::NotFound("No task found with this name.".to_string())),
        }
    }

    pub(crate) fn rearm_task(&self, name: &str, duration: Duration) -> Result<(), Error> {
        let mut state = self.shared.tasks.lock().unwrap();
        let task = state.entries.get_mut(name).unwrap();
        task.reset(duration)?;
        Ok(())
    }

    pub(crate) fn set_task(
        &self,
        name: String,
        command: Arc<dyn Command + Send + Sync>,
        expire: u64,
    ) {
        let mut tasks = self.shared.tasks.lock().unwrap();

        let duration = Duration::new(expire, 0);

        // `Instant` at which the key expires.
        let when = Instant::now() + duration;

        // Only notify the worker task if the newly inserted expiration is the
        // **next** task to run.
        let notify = tasks
            .next_expiration()
            .map(|expiration| expiration > when)
            .unwrap_or(true);

        // Track the expiration.
        tasks.expirations.insert((when, name.to_string().clone()));

        // Insert into the `HashMap`.
        let task = Task::new(command, duration);
        let prev = tasks.entries.insert(name.clone(), task);

        // If there was a value previously associated with the key
        // The associated entry in the `expirations` map must also be removed.
        if let Some(prev) = prev {
            tasks
                .expirations
                .remove(&(prev.runs_at, name.to_string().clone()));
        }

        println!("----- STATE -----");
        println!("{:#?}", tasks);
        println!("----- END STATE -----");

        drop(tasks);

        if notify {
            // Notify the background task if it needs to update its state to reflect a new expiration.
            self.shared.notify_task_runner.notify_one();
        }
    }

    pub(crate) fn get_user(&self) -> Result<User, Error> {
        Ok(self.shared.user.clone())
    }
}

struct Shared {
    user: User,
    tasks: Mutex<Tasks>,
    notify_task_runner: tokio::sync::Notify,
}

impl Shared {
    /// Run the command and return the `Instant` at which the **next**
    /// task will run. The background task will sleep until this instant.
    fn run_action(&self) -> Option<Instant> {
        let mut state = self.tasks.lock().unwrap();

        let state = &mut *state;

        // Find all keys scheduled to expire **before** now.
        let now = Instant::now();

        while let Some(&(when, ref key)) = state.expirations.iter().next() {
            if when > now {
                // Done purging, `when` is the instant at which the next key
                // expires. The worker task will wait until this instant.
                return Some(when);
            }

            // The task clock has reached 0:
            // 1. run the action and,
            // 2. remove it from the expiration table.
            state.entries.get_mut(key)?.run();
            state.expirations.remove(&(when, key.clone()));
        }

        None
    }
}

#[derive(Clone, Debug)]
struct Tasks {
    entries: HashMap<String, Task>,
    expirations: BTreeSet<(Instant, String)>,
}

impl Tasks {
    fn next_expiration(&self) -> Option<Instant> {
        self.expirations
            .iter()
            .next()
            .map(|expiration| expiration.0)
    }
}

async fn run_expired_tasks(shared: Arc<Shared>) {
    loop {
        if let Some(when) = shared.run_action() {
            println!("run_expired_tasks(): wait until the next task run.");
            tokio::select! {
                _ = time::sleep_until(when) => {}
            _ = shared.notify_task_runner.notified() => {}
            }
        } else {
            shared.notify_task_runner.notified().await;
        }
    }
}
