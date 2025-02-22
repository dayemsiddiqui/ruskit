use clap::Parser;
use crate::app::console::kernel;

#[derive(Parser)]
#[command(name = "schedule")]
pub struct ScheduleCommand {
    #[arg(short, long)]
    daemon: bool,
}

impl ScheduleCommand {
    pub async fn handle(&self) {
        println!("Starting scheduler...");
        
        // Initialize the kernel
        kernel::schedule().await;
    }
} 