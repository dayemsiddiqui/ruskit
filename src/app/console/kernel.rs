use crate::framework::schedule::scheduler;
use chrono::NaiveTime;

pub async fn schedule() {
    let mut sched = scheduler().lock().await;
    println!("Initializing scheduler tasks...");

    // Run every minute
    match sched
        .task("minute-task", || {
            println!("Running task every minute");
        })
        .everyMinute() {
        Ok(task) => sched.add_task(task).await,
        Err(e) => eprintln!("Failed to create minute task: {:?}", e),
    }

    // Run every 5 minutes
    match sched
        .task("five-minute-task", || {
            println!("Running task every 5 minutes");
        })
        .everyFiveMinutes() {
        Ok(task) => sched.add_task(task).await,
        Err(e) => eprintln!("Failed to create five-minute task: {:?}", e),
    }

    // Run hourly at minute 30
    match sched
        .task("hourly-task", || {
            println!("Running task every hour at minute 30");
        })
        .hourlyAt(30) {
        Ok(task) => sched.add_task(task).await,
        Err(e) => eprintln!("Failed to create hourly task: {:?}", e),
    }

    // Run daily at 3:00 PM
    let daily_time = NaiveTime::from_hms_opt(15, 0, 0).unwrap();
    match sched
        .task("daily-task", || {
            println!("Running task daily at 3:00 PM");
        })
        .dailyAt(daily_time) {
        Ok(task) => sched.add_task(task).await,
        Err(e) => eprintln!("Failed to create daily task: {:?}", e),
    }

    // Run every Monday at 8:00 AM
    let weekly_time = NaiveTime::from_hms_opt(8, 0, 0).unwrap();
    match sched
        .task("weekly-task", || {
            println!("Running task every Monday at 8:00 AM");
        })
        .weeklyOn(1, weekly_time) {
        Ok(task) => sched.add_task(task).await,
        Err(e) => eprintln!("Failed to create weekly task: {:?}", e),
    }

    // Run on the first day of every month at midnight
    match sched
        .task("monthly-task", || {
            println!("Running task on the first day of every month");
        })
        .monthly() {
        Ok(task) => sched.add_task(task).await,
        Err(e) => eprintln!("Failed to create monthly task: {:?}", e),
    }

    // Run with custom cron expression (every 15 minutes)
    match sched
        .task("custom-task", || {
            println!("Running task with custom schedule (every 15 minutes)");
        })
        .cron("0 */15 * * * *") {
        Ok(task) => sched.add_task(task).await,
        Err(e) => eprintln!("Failed to create custom task: {:?}", e),
    }

    println!("Finished initializing scheduler tasks");
    
    // Run the scheduler
    sched.run().await;
} 