use std::os::unix::net::{SocketAddr, UnixListener, UnixStream};
use std::{env, fs, process};
use std::sync::LazyLock;

use services::Service;
use sockets::Socket;
use targets::Target;

mod sockets;
mod services;
mod targets;
mod utils;

static USER_MODE: LazyLock<bool> = LazyLock::new(|| process::id() != 1);
static PATH_PREFIX: LazyLock<&str> = LazyLock::new(|| if *USER_MODE {
    let runtime_dir: String = env::var("XDG_RUNTIME_DIR").expect("XDG_RUNTIME_DIR was not set!");
    let runtime_dir: &'static str = Box::leak(runtime_dir.into_boxed_str());
    runtime_dir
} else {
    "/run/"
});

static SOCKET_PATH: LazyLock<&str> = LazyLock::new(|| {
    let socket_path = format!("{}{}", *PATH_PREFIX, "/sysmgr.sock");
    Box::leak(socket_path.into_boxed_str())
});



fn main() {
    println!("sysmgr v0 - Development build! Here be dragons!");
    println!("User mode: {:?}", *USER_MODE);
    
    println!("Attempting to reach default.target");


    if fs::metadata(*SOCKET_PATH).is_ok() {
        println!("A socket is already present. Deleting...");
        fs::remove_file(*SOCKET_PATH)
            .expect(&(format!("{}{}", "could not delete previous socket at ", *SOCKET_PATH)));
    }

    let unix_listener = UnixListener::bind(*SOCKET_PATH).expect("Could not create the unix socket?");

    let global_state = GlobalState {
        services: Vec::new(),
        sockets: Vec::new(),
        targets: Vec::new(),
    };

    // Load all systemd units
    utils::systemd_load_all();

    println!("Ready!");

    loop {
        let (mut unix_stream, socket_address) = unix_listener
            .accept()
            .expect("Failed at accepting a connection on the unix listener");
        handle_connection(unix_stream, socket_address);
    }
}

fn handle_connection(mut stream: UnixStream, socket_addr: SocketAddr) {
    // to be filled
}

struct GlobalState {
    services: Vec<Service>,
    sockets: Vec<Socket>,
    targets: Vec<Target>
}