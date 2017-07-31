#[macro_use]
extern crate rouille;

use std::io;
use std::env;
use std::process::Command;
use std::thread;
use std::time::Duration;
use rouille::cgi::CgiRun;

fn main() {
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
