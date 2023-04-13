use regex::Captures;
use regex::Regex;
use serde::{Deserialize, Deserializer, Serialize};
use serde_yaml;
use std::collections::HashMap;
use std::env;

use crate::errors::TaruError;

pub type Jobs = HashMap<String, Job>;
pub type Variables = HashMap<String, String>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub steps: Vec<String>,
    #[serde(rename = "requires", default)]
    pub prerequisites: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BaseSchema {
    jobs: Jobs,
    #[serde(deserialize_with = "deserialize_variables", default)]
    variables: Variables,
}

/// Variable entries in the YAML that refers to an environment variable are replaced
/// with the corresponding value for that environment variable.
///
/// # Examples
///
/// If the environment variable SHELL=/bin/zsh then
/// variables:
///     var-one: env.SHELL
/// ------> will be converted into:
/// variables:
///     var-one: /bin/zsh
fn deserialize_variables<'a, D>(deserializer: D) -> Result<Variables, D::Error>
where
    D: Deserializer<'a>,
{
    let mut map: Variables = HashMap::deserialize(deserializer)?;
    for (_, value) in map.iter_mut() {
        if value.starts_with("env.") {
            let mut stripped = value.clone();
            stripped = stripped.trim_start_matches("env.").to_string();
            stripped = env::var(&stripped).unwrap_or_else(|_| stripped);
            *value = stripped;
        }
    }
    return Ok(map);
}

/// Given a `jobs` object, replace every variable, specified using double curly braces
/// ({{variable}}), in any step of the job with it's
/// corresponding value in the variables map.
///
/// # Examples
///
/// A job step command:
///     ls {{tmp_dir}}
/// A varaible map:
///     tmp_dir: /tmp
///
/// Results in the following step command:
///     ls /tmp
///

fn inject_variables_into_jobs(jobs: &Jobs, variables: &Variables) -> Jobs {
    let mut injected_jobs = jobs.clone();
    for (_, job) in injected_jobs.iter_mut() {
        let re = Regex::new(r"\{\{ *?(\w+?) *?\}\}").unwrap();
        for step in job.steps.iter_mut() {
            let res = re.replace_all(step, |caps: &Captures| {
                format!(
                    "{}",
                    variables.get(&caps[1]).expect("Invalid variable name")
                )
            });
            *step = res.to_string();
        }
    }
    injected_jobs
}

// Read the YAML and inject the variables into the shell commands.
//
// # Arguments
//
// * `path` The path to the YAML file, as a string value.
pub fn parse_jobs(path: &str) -> Result<Jobs, TaruError> {
    let yaml_str = std::fs::read_to_string(path)?;
    let root: BaseSchema =
        serde_yaml::from_str(&yaml_str).or_else(|_| Err(TaruError::ParseError))?;

    let injected_jobs = inject_variables_into_jobs(&root.jobs, &root.variables);
    Ok(injected_jobs)
}
