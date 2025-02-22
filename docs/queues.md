# Queue System

The Ruskit queue system provides a robust way to defer time-consuming tasks, improving your application's response times and overall user experience. This guide covers everything you need to know about working with queues in Ruskit.

## Table of Contents
- [Introduction](#introduction)
- [Configuration](#configuration)
- [Creating Jobs](#creating-jobs)
- [Dispatching Jobs](#dispatching-jobs)
- [Running the Queue Worker](#running-the-queue-worker)
- [Job Lifecycle](#job-lifecycle)
- [Real-World Use Cases](#real-world-use-cases)

## Introduction

Queues allow you to defer time-consuming tasks until a later time, such as:
- Sending emails
- Processing large files
- Making API calls
- Generating reports
- Processing images
- Sending notifications

By moving these operations to a queue, your application can respond to requests quickly while processing these tasks in the background.

## Configuration

The queue system is configured in your application's bootstrap process. By default, it uses a SQLite database as the queue driver, but it also supports other drivers like SQS (Amazon Simple Queue Service).

```rust
// Example queue configuration
let queue_config = QueueConfig {
    driver: QueueDriverType::Database,
    default_queue: "default".to_string(),
    // ... other configuration options
};
```

## Creating Jobs

To create a job, implement the `Job` trait:

```rust
use serde::{Serialize, Deserialize};
use async_trait::async_trait;
use crate::framework::queue::Job;

#[derive(Debug, Serialize, Deserialize)]
pub struct SendEmailJob {
    pub to: String,
    pub subject: String,
    pub body: String,
}

#[async_trait]
impl Job for SendEmailJob {
    fn queue(&self) -> Option<String> {
        Some("emails".to_string()) // Custom queue name
    }

    fn delay(&self) -> Option<u64> {
        Some(30) // Delay in seconds
    }

    fn tries(&self) -> Option<u32> {
        Some(3) // Maximum retry attempts
    }

    async fn handle(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Implement email sending logic here
        println!("Sending email to: {}", self.to);
        Ok(())
    }

    fn serialize(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(serde_json::to_string(self)?)
    }

    fn deserialize(data: &str) -> Result<Box<dyn Job>, Box<dyn std::error::Error>> {
        let job: SendEmailJob = serde_json::from_str(data)?;
        Ok(Box::new(job))
    }

    fn type_name() -> String {
        "SendEmailJob".to_string()
    }
}
```

## Dispatching Jobs

You can dispatch jobs from anywhere in your application using the `Queue::dispatch` method:

```rust
use crate::framework::queue::Queue;

// In your controller or service
async fn register_user(user: User) -> Result<(), Error> {
    // Save user to database
    user.save().await?;

    // Dispatch welcome email job
    let email_job = SendEmailJob {
        to: user.email,
        subject: "Welcome to our platform!".to_string(),
        body: "Thank you for registering...".to_string(),
    };
    
    Queue::dispatch(email_job).await?;
    Ok(())
}
```

## Running the Queue Worker

To process queued jobs, you need to run a queue worker. Ruskit provides a CLI command for this:

```bash
# Run worker for default queue
cargo run --bin ruskit -- queue:work

# Run worker for specific queue
cargo run --bin ruskit -- queue:work --queue emails

# Run worker with custom settings
cargo run --bin ruskit -- queue:work --queue emails --sleep 5 --tries 3
```

## Job Lifecycle

1. **Creation**: Jobs are created by implementing the `Job` trait
2. **Dispatching**: Jobs are serialized and stored in the queue
3. **Processing**: Queue worker picks up jobs and executes them
4. **Completion/Failure**: Jobs are either completed successfully or failed and retried
5. **Cleanup**: Completed jobs are removed from the queue

## Real-World Use Cases

### 1. User Registration Flow
```rust
async fn register_user(mut ctx: Context) -> Result<Response, Error> {
    let user: User = ctx.extract::<User>().await?;
    
    // Save user synchronously
    user.save().await?;
    
    // Queue async tasks
    Queue::dispatch(SendWelcomeEmailJob {
        user_id: user.id,
        email: user.email.clone(),
    }).await?;
    
    Queue::dispatch(SetupUserWorkspaceJob {
        user_id: user.id,
    }).await?;
    
    Queue::dispatch(NotifyAdminsJob {
        event: "new_user_registered",
        user_id: user.id,
    }).await?;
    
    Ok(Response::json(user))
}
```

### 2. File Processing System
```rust
async fn upload_video(mut ctx: Context) -> Result<Response, Error> {
    let video: UploadedFile = ctx.extract::<UploadedFile>().await?;
    
    // Save file metadata synchronously
    let video_record = video.save_metadata().await?;
    
    // Queue processing tasks
    Queue::dispatch(VideoTranscodingJob {
        video_id: video_record.id,
        formats: vec!["mp4", "webm"],
    }).await?;
    
    Queue::dispatch(GenerateThumbnailsJob {
        video_id: video_record.id,
    }).await?;
    
    Queue::dispatch(ExtractMetadataJob {
        video_id: video_record.id,
    }).await?;
    
    Ok(Response::json(video_record))
}
```

### 3. E-commerce Order Processing
```rust
async fn place_order(mut ctx: Context) -> Result<Response, Error> {
    let order: Order = ctx.extract::<Order>().await?;
    
    // Save order synchronously
    order.save().await?;
    
    // Queue async tasks
    Queue::dispatch(SendOrderConfirmationJob {
        order_id: order.id,
        email: order.customer_email.clone(),
    }).await?;
    
    Queue::dispatch(UpdateInventoryJob {
        order_id: order.id,
        items: order.items.clone(),
    }).await?;
    
    Queue::dispatch(NotifyShippingDepartmentJob {
        order_id: order.id,
    }).await?;
    
    Queue::dispatch(ProcessPaymentJob {
        order_id: order.id,
        amount: order.total,
        payment_method: order.payment_method,
    }).await?;
    
    Ok(Response::json(order))
}
```

### 4. Scheduled Report Generation
```rust
async fn schedule_report(mut ctx: Context) -> Result<Response, Error> {
    let report_config: ReportConfig = ctx.extract::<ReportConfig>().await?;
    
    // Save report configuration
    let config = report_config.save().await?;
    
    // Queue report generation with delay
    let job = GenerateReportJob {
        config_id: config.id,
        report_type: config.report_type,
        parameters: config.parameters,
    };
    
    Queue::dispatch(job).await?;
    
    Ok(Response::json(config))
}
```

### 5. Notification System
```rust
async fn trigger_notification(event: Event) -> Result<(), Error> {
    // Queue different notification types
    Queue::dispatch(SendPushNotificationJob {
        event_id: event.id,
        user_ids: event.subscriber_ids.clone(),
    }).await?;
    
    Queue::dispatch(SendEmailNotificationJob {
        event_id: event.id,
        user_ids: event.subscriber_ids.clone(),
    }).await?;
    
    Queue::dispatch(SendSmsNotificationJob {
        event_id: event.id,
        user_ids: event.subscriber_ids.clone(),
    }).await?;
    
    Ok(())
}
``` 