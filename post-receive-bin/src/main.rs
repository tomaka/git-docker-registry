use std::ffi::OsString;
use std::fs;
use std::process::Command;

// FIXME: needs locking to avoid executing multiple times simultaneously

fn main() {
    println!("Updating the git repository clone");
    let status = Command::new("git")
        .env_clear()
        .current_dir("/home/local-clone")
        .arg("fetch")
        .arg("origin")
        .status()
        .unwrap();
    if !status.success() {
        return;
    }
    let status = Command::new("git")
        .env_clear()
        .current_dir("/home/local-clone")
        .arg("reset")
        .arg("--hard")
        .arg("origin/master")
        .status()
        .unwrap();
    if !status.success() {
        return;
    }

    println!("Trying to find docker files");

    let mut any_found = false;
    for image in fs::read_dir("/home/local-clone").unwrap() {
        let image = image.unwrap();
        if !image.metadata().unwrap().is_dir() {
            continue;
        }

        if !image.path().join("Dockerfile").exists() {
            continue;
        }

        any_found = true;

        println!("Building Dockerfile in directory `{}`", image.file_name().to_string_lossy());
        let mut image_name: OsString = "localhost:5000/".to_owned().into();
        image_name.push(image.file_name());
        let status = Command::new("docker")
            .arg("build")
            .arg("-t")
            .arg(image_name.clone())
            .arg(".")
            .current_dir(image.path())
            .status()
            .unwrap();
        if !status.success() {
            continue;
        }
        let status = Command::new("docker")
            .arg("push")
            .arg(image_name)
            .status()
            .unwrap();
        if !status.success() {
            continue;
        }
    }

    if !any_found {
        println!("No Dockerfile found in git repository");
    }
}
