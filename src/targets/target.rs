use crate::utils::{start_dependency, Dependency};

pub struct Target {
    depends: Vec<Dependency>,
}

pub fn start_target(target: Target) -> Result<(), String> {
    for dependency in target.depends {
        start_dependency(dependency).expect("Ruh roh, failure");
    }

    Ok(())
}
