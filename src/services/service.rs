use std::process::Command;

use crate::utils::Dependency;

struct BashCommand {
    cmd: String,
    args: String,
}

pub struct Service {
    depends: Option<Box<Dependency>>,
    name: String,
    description: String,
    documentation: Vec<String>,
    exec_start: BashCommand,
    exec_stop: BashCommand,
}

pub fn start_service(service: Service) -> Result<(), String> {
    let mut command = Command::new(service.exec_start.cmd);
    command.args(service.exec_start.args.split_whitespace());

    Ok(())
}
