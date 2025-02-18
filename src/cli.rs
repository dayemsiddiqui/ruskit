use clap::{Parser, Subcommand};
use console::style;
use std::{path::Path, fs, process::{self, Stdio}};
use tokio::{process::{Command, Child}, fs::File, sync::broadcast};
use tokio::sync::mpsc;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use walkdir::WalkDir;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use nix::unistd::{self, Pid};
use nix::sys::signal::{self, Signal};
use std::sync::Arc;

static RUNNING: AtomicBool = AtomicBool::new(true);
static CHILD_PID: AtomicI32 = AtomicI32::new(0);

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// The subcommand name that cargo adds (we ignore this)
    #[clap(skip)]
    cargo_subcommand_name: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the server
    Serve,
    /// Start the server in development mode with hot reloading
    Dev,
}

fn clean_template_artifacts() {
    // Clean the generated template files
    for entry in WalkDir::new("target")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .to_str()
                .map(|s| s.contains("askama_generated"))
                .unwrap_or(false)
        })
    {
        let _ = fs::remove_file(entry.path());
    }
}

fn kill_all_child_processes() {
    // First try SIGTERM
    let _ = process::Command::new("sh")
        .arg("-c")
        .arg("ps -o pid --ppid $$ --no-headers | xargs -r kill -TERM")
        .output();

    // Give processes a chance to terminate gracefully
    std::thread::sleep(std::time::Duration::from_millis(100));

    // Then force kill any remaining processes
    let _ = process::Command::new("sh")
        .arg("-c")
        .arg("ps -o pid --ppid $$ --no-headers | xargs -r kill -9")
        .output();

    // Also kill any cargo processes
    let _ = process::Command::new("pkill")
        .arg("-9")
        .arg("-f")
        .arg("cargo")
        .output();
}

async fn restart_server() -> Child {
    clean_template_artifacts();

    println!("{}", style("Building project...").yellow());
    let build_output = Command::new("cargo")
        .arg("build")
        .output()
        .await
        .expect("Failed to run cargo build");

    if !build_output.status.success() {
        println!("{}", style("Build failed, waiting for changes...").red());
        let error = String::from_utf8_lossy(&build_output.stderr);
        println!("{}", style(error).red());
    }

    println!("{}", style("Starting Ruskit server...").cyan());
    let child = Command::new("cargo")
        .arg("run")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .kill_on_drop(true)
        .process_group(0)
        .spawn()
        .expect("Failed to start server");
    
    CHILD_PID.store(child.id().unwrap_or(0) as i32, Ordering::SeqCst);
    child
}

async fn watch_directory(path: &str, shutdown_tx: broadcast::Sender<()>) 
    -> notify::Result<(RecommendedWatcher, mpsc::Receiver<notify::Result<notify::Event>>)> 
{
    let (tx, rx) = mpsc::channel(32); // Increased channel capacity

    let mut watcher = notify::recommended_watcher(move |res| {
        if RUNNING.load(Ordering::SeqCst) {
            match tx.try_send(res) {
                Ok(_) => {},
                Err(e) => eprintln!("Failed to send notify event: {}", e),
            }
        }
    })?;

    watcher.watch(Path::new(path), RecursiveMode::Recursive)?;
    Ok((watcher, rx))
}

async fn handle_shutdown(mut server_process: Child) {
    RUNNING.store(false, Ordering::SeqCst);
    println!("{}", style("\nShutting down...").red());
    
    // Try graceful shutdown first
    let _ = server_process.kill().await;
    let _ = server_process.wait().await;
    
    // Kill all remaining child processes
    kill_all_child_processes();
}

pub async fn run() {
    let (shutdown_tx, _) = broadcast::channel(1);
    let shutdown_tx_clone = shutdown_tx.clone();

    // Set up signal handlers
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    tokio::spawn(async move {
        if let Ok(()) = tokio::signal::ctrl_c().await {
            r.store(false, Ordering::SeqCst);
            let _ = shutdown_tx_clone.send(());
        }
    });

    let args: Vec<String> = std::env::args().enumerate()
        .filter(|(i, arg)| *i != 1 || arg != "kit")
        .map(|(_, arg)| arg)
        .collect();

    let cli = Cli::parse_from(args);

    match cli.command {
        Commands::Serve => {
            println!("{}", style("Starting Ruskit server...").cyan());
            let mut child = Command::new("cargo")
                .arg("run")
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .kill_on_drop(true)
                .process_group(0)
                .spawn()
                .expect("Failed to start server");
            
            CHILD_PID.store(child.id().unwrap_or(0) as i32, Ordering::SeqCst);

            tokio::select! {
                _ = child.wait() => {},
                _ = tokio::signal::ctrl_c() => {
                    handle_shutdown(child).await;
                }
            }
        }
        Commands::Dev => {
            println!("{}", style("Starting Ruskit development server...").cyan());
            println!("{}", style("Hot reloading enabled").yellow());

            let mut server_process = restart_server().await;
            let mut shutdown_rx = shutdown_tx.subscribe();

            let (src_watcher, mut src_rx) = watch_directory("src", shutdown_tx.clone()).await
                .expect("Failed to watch src directory");
            let (templates_watcher, mut templates_rx) = watch_directory("templates", shutdown_tx.clone()).await
                .expect("Failed to watch templates directory");

            println!("{}", style("Watching project directories...").yellow());

            loop {
                tokio::select! {
                    Ok(()) = shutdown_rx.recv() => {
                        handle_shutdown(server_process).await;
                        break;
                    }
                    Some(event) = src_rx.recv() => {
                        if event.is_ok() && running.load(Ordering::SeqCst) {
                            println!("{}", style("Source changes detected, rebuilding...").yellow());
                            let _ = server_process.kill().await;
                            server_process = restart_server().await;
                        }
                    }
                    Some(event) = templates_rx.recv() => {
                        if event.is_ok() && running.load(Ordering::SeqCst) {
                            println!("{}", style("Template changes detected, rebuilding...").yellow());
                            let _ = server_process.kill().await;
                            server_process = restart_server().await;
                        }
                    }
                    else => break
                }
            }

            // Clean up
            drop(src_watcher);
            drop(templates_watcher);
            
            // Final cleanup
            kill_all_child_processes();
            process::exit(0);
        }
    }
} 