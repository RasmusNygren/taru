# Taru - local workflow runner

![Build status](https://github.com/RasmusNygren/taru/actions/workflows/main.yml/badge.svg)

Taru is a simple local-first workflow-runner for unix-based systems.
Taru is designed as an alternative to makefiles, shell scripts and bash aliases to store and manage
local tasks and workflows.
The configuration is specified through YAML files,
and allows for the execution of any shell commands that can be run in your local shell environment.




## Features
- Workflow configuration using YAML with support for injecting:
    - User-defined variables
    - Environment variables

- Specify multi-depth job dependencies.

## Installation
Running the application requires Rust to be installed on your system and the
installation guide requires the Cargo
package manager to be accessible on your `PATH`.

### Building using Cargo
Run
```bash
cargo install --git https://github.com/RasmusNygren/taru taru
```

Assuming that your cargo `bin` directory is on your path you
can now invoke the Taru cli through `taru`.


### Using homebrew
Taru is available via the `rasmusnygren/taru` tap.

Run
```shell
brew tap rasmusnygren/taru
brew install taru
```

## Using Taru
```shell
Usage: taru <COMMAND>

Commands:
  `run`   Run a job
  `list`  List all jobs
  `help`  Print this message or the help of the given subcommand(s)

Options:
  `-h`, `--help`     Print help
  `-V`, `--version`  Print version
```

### YAML Syntax
The YAML schema currently supports `jobs` and `variables` as top-level keywords.
#### `jobs`
Each job is specified with a name and must include a `steps` parameter where each step
must correspond to a valid shell command, but a command can also include variables
if they are specified in the `variables` section. Variables are referred to by specifying the key value
inside single brackets e.g. `{variable_name}`

Each job also accepts
`requires` as an optional parameter to specify dependencies. `requires` only accepts other jobs as depdendencies
and thus only other job names are valid values.
#### `variables`
Variables are optional and are specified using
`key: value` syntax.


## Examples:
```yaml
jobs:
  build-dev:
    requires:
      - dependency
    steps:
      - cargo build
      - chmod +x {bin_source}
      - cp {bin_source} {bin_output}
  dependency:
    steps:
      - echo 'This is a dependency and should run first'

variables:
  bin_output: /opt/homebrew/bin/
  bin_source: ./target/debug/taru
```
