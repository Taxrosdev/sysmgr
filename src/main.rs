use std::os::unix::net::{SocketAddr, UnixListener, UnixStream};
use std::process::Command;
use std::{env, fs, process};

use std::sync::LazyLock;

static USER_MODE: LazyLock<bool> = LazyLock::new(|| process::id() != 1);
static SOCKET_PATH: LazyLock<&str> = LazyLock::new(|| if *USER_MODE {
    let runtime_dir: String = env::var("XDG_RUNTIME_DIR").expect("XDG_RUNTIME_DIR was not set! Panicking!") + "/sysmgr.sock";
    let runtime_dir: &'static str = Box::leak(runtime_dir.into_boxed_str());
    runtime_dir
} else {
    "/run/sysmgr.sock"
});

fn main() {
    println!("sysmgr v0 - Development build! Here be dragons!");
    println!("User mode: {:?}", *USER_MODE);
    
    println!("Attempting to reach default.target");


    if fs::metadata(*SOCKET_PATH).is_ok() {
        println!("A socket is already present. Deleting...");
        fs::remove_file(*SOCKET_PATH)
            .expect(&("could not delete previous socket at ".to_owned() + *SOCKET_PATH));
    }

    let unix_listener = UnixListener::bind(*SOCKET_PATH).expect("Could not create the unix socket?");

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
    println!("{:?}", socket_addr)
}

struct BashCommand {
    cmd: String,
    args: String,
}

struct Service {
    depends: Dependency,
    name: String,
    description: String,
    documentation: Vec<String>,
    exec_start: BashCommand,
    exec_stop: BashCommand,
}

enum Dependency {
    Target(Box<Target>),
    Service(Box<Service>),
    Socket(Box<Socket>),
}

struct Target {
    depends: Dependency,
}

struct Socket {
    depends: Dependency,
    service: Service,
}

fn start_service(service: Service) {
    let mut command = Command::new(service.exec_start.cmd);
    command.args(service.exec_start.args.split_whitespace());
}
