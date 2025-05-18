use crate::{services::{start_service, Service}, utils::Dependency};

pub struct Socket {
    service: Service,
}

pub fn start_socket(socket: Socket) -> Result<(), String> {
    start_service(socket.service)
}
