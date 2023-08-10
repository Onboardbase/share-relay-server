extern crate daemonize;

use std::fs::File;

use daemonize::Daemonize;

pub fn start() {
    let stdout = File::create("/tmp/secureshare.out").unwrap();
    let stderr = File::create("/tmp/secureshare.err").unwrap();

    let daemonize = Daemonize::new()
        .pid_file("/tmp/secureshare.pid") // Every method except `new` and `start`
        .chown_pid_file(true) // is optional, see `Daemonize` documentation
        .working_directory("/tmp") // for default behaviour.
        .user("nobody")
        .group("daemon") // Group name
        .group(2) // or group id.
        .umask(0o777) // Set umask, `0o027` by default.irect stderr to `/tmp/daemon.err`.
        .stdout(stdout)
        .stderr(stderr)
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(_) => println!("Success, daemonized"),
        Err(e) => eprintln!("Error, {}", e),
    }
}
