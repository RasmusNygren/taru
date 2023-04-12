use crate::parser::{parse_jobs, Jobs};
use crate::TaruError;
use crate::Config;

pub fn process_job(job_name: &str, config: &Config) -> Result<(), TaruError> {
    let jobs = parse_jobs(&config.file_path)?;
    run_job(job_name, &jobs)?;
    Ok(())
}

/// Runs a job - ensuring that all the requirements for a given job is run
/// prior to the main job by recursively calling the jobs specified as prerequisites.
///
/// Supports nested requirements.
///
/// # Arguments
///
/// * `job_name` - The job name to run, must match a job defined in `jobs`.
/// * `jobs` - A map of jobs, identified by their name.
fn run_job(job_name: &str, jobs: &Jobs) -> Result<(), TaruError> {
    let job = jobs.get(job_name).ok_or(TaruError::InvalidJob)?;
    if !job.prerequisites.is_empty() {
        for req in &job.prerequisites {
            run_job(req, jobs)?;
        }
    }
    run_steps_in_job(&job.steps)?;
    Ok(())
}

fn get_shell_envvar_or_default(default: &str) -> String {
    std::env::var("SHELL").unwrap_or_else(|_| default.to_string())
}

/// Executes every linux shell command that was specified in the YAML configuration.
///
/// # Arguments
///
/// * `steps` A vector with linux shell commands
///
/// # Returns
///
/// * `TaruError` if any of the commands did not execute successfully.
fn run_steps_in_job(steps: &Vec<String>) -> Result<(), TaruError> {

    let shell = get_shell_envvar_or_default("sh");
    for step in steps {
        let validated_cmd = std::process::Command::new(&shell)
            .arg("-c")
            .arg("-n")
            .arg(step)
            .status()
            .expect("Shell command validation failed");
        if !validated_cmd.success() {
            return Err(TaruError::RuntimeError);
        }
    }

    for step in steps {
        log::info!("Running step: {:?}", step);
        
        let command = std::process::Command::new(&shell)
            .arg("-c")
            .arg(step)
            .output()
            .expect("Failed to run the command '{step}'");

        if !command.status.success() {
            return Err(TaruError::RuntimeError);
        }

        let sout = String::from_utf8(command.stdout).unwrap();
        if !sout.is_empty() {
            log::info!("Output: {}", sout);
        }
    }
    Ok(())
}

/// Fetches the key values for all jobs that has been identified in the YAML file.
///
/// # Arguments 
///
/// * `file_path` The path to the YAML file.
///
/// # Returns
///
/// A vector of all the job names.

pub fn get_job_names(file_path: &str) -> Result<Vec<String>, TaruError> {
    let jobs = parse_jobs(file_path)?;
    let vec: Vec<String> = jobs.into_keys().collect();
    Ok(vec)
}
