use crate::{services::{start_service, Service}, sockets::{start_socket, Socket}, targets::{start_target, Target}};

pub enum Dependency {
    Target(Box<Target>),
    Service(Box<Service>),
    Socket(Box<Socket>),
}

pub fn start_dependency(dependency: Dependency) -> Result<(), String> {{
    match dependency {
        Dependency::Target(target) => start_target(*target),
        Dependency::Socket(socket) => start_socket(*socket),
        Dependency::Service(service) => start_service(*service),
    }
}}