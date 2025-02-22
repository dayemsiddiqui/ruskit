use std::time::Duration;
use tokio::time;
use chrono::{DateTime, Local, Timelike, NaiveTime};
use tokio_cron_scheduler::{Job, JobScheduler};
use std::sync::Arc;
use tokio::sync::Mutex as TokioMutex;
use lazy_static::lazy_static;

#[derive(Debug)]
pub enum SchedulerError {
    InvalidCronExpression(String),
    InvalidTime(String),
}

pub struct TaskBuilder {
    name: String,
    command: Arc<dyn Fn() + Send + Sync>,
    timezone: String,
}

impl TaskBuilder {
    pub fn new<F>(name: &str, command: F) -> Self
    where
        F: Fn() -> () + Send + Sync + 'static,
    {
        Self {
            name: name.to_string(),
            command: Arc::new(command),
            timezone: "UTC".to_string(),
        }
    }

    pub fn cron(self, expression: &str) -> Result<Task, SchedulerError> {
        let command = self.command.clone();
        Task::new(&self.name, expression, move || {
            command();
        })
    }

    pub fn everyMinute(self) -> Result<Task, SchedulerError> {
        let command = self.command.clone();
        Task::new(&self.name, "0 * * * * *", move || {
            command();
        })
    }

    pub fn everyTwoMinutes(self) -> Result<Task, SchedulerError> {
        let command = self.command.clone();
        Task::new(&self.name, "0 */2 * * * *", move || {
            command();
        })
    }

    pub fn everyThreeMinutes(self) -> Result<Task, SchedulerError> {
        let command = self.command.clone();
        Task::new(&self.name, "0 */3 * * * *", move || {
            command();
        })
    }

    pub fn everyFiveMinutes(self) -> Result<Task, SchedulerError> {
        let command = self.command.clone();
        Task::new(&self.name, "0 */5 * * * *", move || {
            command();
        })
    }

    pub fn everyTenMinutes(self) -> Result<Task, SchedulerError> {
        let command = self.command.clone();
        Task::new(&self.name, "0 */10 * * * *", move || {
            command();
        })
    }

    pub fn everyFifteenMinutes(self) -> Result<Task, SchedulerError> {
        let command = self.command.clone();
        Task::new(&self.name, "0 */15 * * * *", move || {
            command();
        })
    }

    pub fn everyThirtyMinutes(self) -> Result<Task, SchedulerError> {
        let command = self.command.clone();
        Task::new(&self.name, "0 */30 * * * *", move || {
            command();
        })
    }

    pub fn hourly(self) -> Result<Task, SchedulerError> {
        let command = self.command.clone();
        Task::new(&self.name, "0 0 * * * *", move || {
            command();
        })
    }

    pub fn hourlyAt(self, minute: u32) -> Result<Task, SchedulerError> {
        let command = self.command.clone();
        Task::new(&self.name, &format!("0 {} * * * *", minute), move || {
            command();
        })
    }

    pub fn daily(self) -> Result<Task, SchedulerError> {
        let command = self.command.clone();
        Task::new(&self.name, "0 0 0 * * *", move || {
            command();
        })
    }

    pub fn dailyAt(self, time: NaiveTime) -> Result<Task, SchedulerError> {
        let command = self.command.clone();
        Task::new(
            &self.name,
            &format!("0 {} {} * * *", time.minute(), time.hour()),
            move || {
                command();
            },
        )
    }

    pub fn weekly(self) -> Result<Task, SchedulerError> {
        let command = self.command.clone();
        Task::new(&self.name, "0 0 0 * * 0", move || {
            command();
        })
    }

    pub fn weeklyOn(self, day: u32, time: NaiveTime) -> Result<Task, SchedulerError> {
        let command = self.command.clone();
        Task::new(
            &self.name,
            &format!("0 {} {} * * {}", time.minute(), time.hour(), day),
            move || {
                command();
            },
        )
    }

    pub fn monthly(self) -> Result<Task, SchedulerError> {
        let command = self.command.clone();
        Task::new(&self.name, "0 0 0 1 * *", move || {
            command();
        })
    }

    pub fn monthlyOn(self, day: u32, time: NaiveTime) -> Result<Task, SchedulerError> {
        let command = self.command.clone();
        Task::new(
            &self.name,
            &format!("0 {} {} {} * *", time.minute(), time.hour(), day),
            move || {
                command();
            },
        )
    }

    pub fn quarterly(self) -> Result<Task, SchedulerError> {
        let command = self.command.clone();
        Task::new(&self.name, "0 0 0 1 */3 *", move || {
            command();
        })
    }

    pub fn yearly(self) -> Result<Task, SchedulerError> {
        let command = self.command.clone();
        Task::new(&self.name, "0 0 0 1 1 *", move || {
            command();
        })
    }

    pub fn timezone(mut self, tz: &str) -> Self {
        self.timezone = tz.to_string();
        self
    }
}

pub struct Task {
    name: String,
    command: Arc<dyn Fn() + Send + Sync>,
    cron_expression: String,
    next_run: Option<DateTime<Local>>,
    last_run: Option<DateTime<Local>>,
}

impl Task {
    pub fn new<F>(name: &str, cron_expression: &str, command: F) -> Result<Self, SchedulerError>
    where
        F: Fn() -> () + Send + Sync + 'static,
    {
        Ok(Self {
            name: name.to_string(),
            command: Arc::new(command),
            cron_expression: cron_expression.to_string(),
            next_run: None,
            last_run: None,
        })
    }

    pub fn should_run(&self) -> bool {
        match self.next_run {
            Some(next_run) => Local::now() >= next_run,
            None => false,
        }
    }

    pub fn execute(&mut self) {
        (self.command)();
        self.last_run = Some(Local::now());
    }

    pub async fn register(self, scheduler: &mut JobScheduler) -> Result<(), SchedulerError> {
        let name = self.name.clone();
        let command = self.command.clone();

        let job = Job::new_async(self.cron_expression.as_str(), move |_uuid, _l| {
            let name = name.clone();
            let command = command.clone();
            Box::pin(async move {
                println!("Running task: {}", name);
                command();
            })
        })
        .map_err(|e| SchedulerError::InvalidCronExpression(e.to_string()))?;

        scheduler.add(job).await.map_err(|e| SchedulerError::InvalidCronExpression(e.to_string()))?;
        Ok(())
    }
}

pub struct Scheduler {
    tasks: Vec<Task>,
    scheduler: Option<JobScheduler>,
}

impl Scheduler {
    pub fn new() -> Self {
        println!("Creating new scheduler");
        Scheduler { 
            tasks: Vec::new(),
            scheduler: None,
        }
    }

    pub async fn add_task(&mut self, task: Task) {
        println!("Adding task: {}", task.name);
        let name = task.name.clone();
        let command = task.command.clone();
        let cron_expression = task.cron_expression.clone();

        if self.scheduler.is_none() {
            self.scheduler = Some(JobScheduler::new().await.unwrap());
        }

        let job = Job::new_async(cron_expression.as_str(), move |_uuid, _l| {
            let name = name.clone();
            let command = command.clone();
            Box::pin(async move {
                println!("Running task: {}", name);
                command();
            })
        }).unwrap();

        self.scheduler.as_mut().unwrap().add(job).await.unwrap();
        self.tasks.push(task);
        println!("Current task count: {}", self.tasks.len());
    }

    pub async fn schedule<F>(&mut self, name: &str, cron_expression: &str, command: F)
    where
        F: Fn() -> () + Send + Sync + 'static,
    {
        match Task::new(name, cron_expression, command) {
            Ok(task) => {
                println!("Scheduling task: {}", name);
                self.add_task(task).await;
            },
            Err(SchedulerError::InvalidCronExpression(expr)) => {
                eprintln!("Error: Invalid cron expression '{}' for task '{}'", expr, name);
            }
            Err(SchedulerError::InvalidTime(msg)) => {
                eprintln!("Error: Invalid time for task '{}': {}", name, msg);
            }
        }
    }

    pub async fn run(&mut self) {
        println!("Scheduler started with {} tasks", self.tasks.len());
        if let Some(scheduler) = &self.scheduler {
            scheduler.start().await.unwrap();
            loop {
                time::sleep(Duration::from_secs(60)).await;
            }
        }
    }

    pub fn task<F>(&mut self, name: &str, command: F) -> TaskBuilder
    where
        F: Fn() -> () + Send + Sync + 'static,
    {
        println!("Creating task builder for: {}", name);
        TaskBuilder::new(name, command)
    }
}

lazy_static! {
    static ref SCHEDULER: TokioMutex<Scheduler> = TokioMutex::new(Scheduler::new());
}

pub fn scheduler() -> &'static TokioMutex<Scheduler> {
    &SCHEDULER
} 