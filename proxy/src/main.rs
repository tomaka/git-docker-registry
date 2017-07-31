#[macro_use]
extern crate rouille;

use std::io;
use std::env;
use std::process::Command;
use std::thread;
use std::time::Duration;
use rouille::cgi::CgiRun;

fn main() {
    // Spawn the registry, then wait some time and check whether the registry is still alive.
    println!("Starting registry in background");
    let registry_child = Command::new("/registry")
        .arg("serve")
        .arg("/registry-config.yml")
        .spawn()
        .unwrap();
    // TODO: try_wait() is unstable in the Rust version of Alpine 3.6
    /*thread::sleep(Duration::from_secs(2));
    if let Some(exit_code) = registry_child.try_wait().unwrap() {
        println!("Registry exited with code {:?}", exit_code);
        return;
    }*/

    println!("Now listening on 0.0.0.0:80");

    rouille::start_server("0.0.0.0:80", move |request| {
        rouille::log(&request, io::stdout(), || {
            let mut cmd = Command::new("git");
            cmd.arg("http-backend");

            cmd.env("GIT_PROJECT_ROOT", "/var/git");
            cmd.env("GIT_HTTP_EXPORT_ALL", "");

            cmd.start_cgi(&request).unwrap()
        })
    });
}
