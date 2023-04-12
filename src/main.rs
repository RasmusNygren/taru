use clap::{Parser, Subcommand};
use log::warn;

pub use crate::runner::get_job_names;
pub use crate::runner::process_job;

use crate::errors::TaruError;

mod errors;
mod parser;
mod runner;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser)]
struct Run {
    #[clap(help = "Name of the job")]
    job_name: String,
    #[clap(long, short, help = "File path to config")]
    file_path: Option<String>,
    #[clap(long = "quiet", short, help = "Omit stdout output")]
    quiet_mode: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a job
    Run(Run),

    /// List all jobs
    List {
        #[clap(long, short, help = "File path to config")]
        file_path: Option<String>,
    },
}

pub struct Config {
    file_path: String,
    quiet_mode: bool,
}

fn get_run_config(run_obj: &Run) -> Config {
    let file_path = get_yaml_path(run_obj.file_path.as_ref());
    Config { file_path, quiet_mode: run_obj.quiet_mode }
}

fn get_yaml_path<T: AsRef<str>>(file_path: Option<T>) -> String {
    let default_config_path = "taru-config.yaml";
    let file_path: String = match file_path {
        Some(e) => e.as_ref().to_string(),
        None => default_config_path.to_string(),
    };
    file_path
}


fn main() {
    match try_main() {
        Err(TaruError::InvalidJob) => {
            warn!("Job does not exist");
        }
        Err(TaruError::ParseError) => {
            warn!("The YAML config file is invalid");
        }
        Err(TaruError::RuntimeError) => {
            warn!("Job failed unexpectedly");
        }
        _ => {}
    };
}

fn try_main() -> Result<(), errors::TaruError> {
    let args = Cli::parse();

    match args.command {
        Commands::Run(run) => {
            let config = get_run_config(&run);
            let mut level = log::LevelFilter::Info;
            if config.quiet_mode {
                level = log::LevelFilter::Warn;
            }
            simple_logger::SimpleLogger::new().without_timestamps().with_level(level).init().unwrap();
            process_job(&run.job_name, &config)?;
            Ok(())
        }
        Commands::List { file_path } => {
            let file_path = get_yaml_path(file_path);
            let jobs = get_job_names(&file_path)?;
            println!("{:?}", jobs);
            Ok(())
        }
    }
}
