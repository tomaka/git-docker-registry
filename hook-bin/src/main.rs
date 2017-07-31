use std::env;
use std::ffi::OsString;
use std::fs;
use std::process;
use std::process::Command;

fn main() {
    let (branch_name, _ref_before, ref_after) = {
        let args = env::args().collect::<Vec<_>>();
        if args.len() < 4 {
            println!("Expected 3 arguments");
            process::exit(1);
        }
        (args[1].clone(), args[2].clone(), args[3].clone())
    };

    println!("Cloning the new reference");
    fs::create_dir_all("/home/local-clone").unwrap();
    let status = Command::new("git")
        .env_clear()
        .arg("--git-dir")
        .arg("/var/git")
        .arg("--work-tree")
        .arg("/home/local-clone")
        .arg("checkout")
        .arg("-q")
        .arg(ref_after)
        .status()
        .unwrap();
    if !status.success() {
        println!("git checkout failed");
        process::exit(1);
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

        let mut image_name: OsString = if let Ok(registry) = env::var("REGISTRY_URL") {
            format!("{}/", registry).into()
        } else {
            OsString::new()
        };
        image_name.push(image.file_name());
        println!("Building Dockerfile in directory `{}` under tag `{}`",
                 image.file_name().to_string_lossy(),
                 image_name.to_string_lossy());
        let status = Command::new("docker")
            .arg("build")
            .arg("-t")
            .arg(image_name.clone())
            .arg(".")
            .current_dir(image.path())
            .status()
            .unwrap();
        if !status.success() {
            process::exit(1);
        }

        if branch_name == "refs/heads/master" {
            if let Ok(registry) = env::var("REGISTRY_URL") {
                println!("Pushing image `{}`", image.file_name().to_string_lossy());
                let status = Command::new("docker")
                    .arg("push")
                    .arg(image_name)
                    .status()
                    .unwrap();
                if !status.success() {
                    println!("Failed to push to registry `{}`", registry);
                    process::exit(1);
                }
            } else {
                println!("Image `{}` not pushed because the `REGISTRY_URL` environment variable \
                          isn't set", image_name.to_string_lossy());
            }
        } else {
            println!("Note: only images on the master branch are pushed to the registry");
        }
    }

    if !any_found {
        println!("No Dockerfile found in git repository");
    }
}
