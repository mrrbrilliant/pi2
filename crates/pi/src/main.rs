#![allow(dead_code)]

pub mod config;
pub mod error;
pub mod pkg;

use pkg::Pkg;

use std::fs::File;
use std::io::prelude::*;

use toml::to_string;
fn main() {
    let mut data = String::new();
    let mut file = File::open("pkgbuild.toml").unwrap();
    file.read_to_string(&mut data).unwrap();

    let toml_data: Pkg = toml::from_str(&data).unwrap();

    if let Some(builder) = toml_data.builder {
        println!("{}", &builder);
        subprocess::Exec::shell(&builder).join().unwrap();
    }
}

fn create_sample() {
    let pkg = Pkg {
        pkgname: String::new(),
        version: String::new(),
        release: 0,
        description: String::new(),
        architectures: Some(vec![String::new()]),
        licenses: Some(vec![String::new()]),
        project_url: Some(vec![String::new()]),
        sources: Some(vec![String::new()]),
        provides: Some(vec![String::new()]),
        conflict: Some(vec![String::new()]),
        replace: Some(vec![String::new()]),
        dependencies: Some(vec![String::new()]),
        build_dependencies: Some(vec![String::new()]),
        optional_dependencies: Some(vec![String::new()]),
        builder: Some(String::new()),
        deploy: Some(String::new()),
        env_vars: Some(vec![String::new()]),
    };
    let data = to_string(&pkg).unwrap();
    let mut file = File::create("pkgbuid.toml").unwrap();
    file.write_all(&data.as_bytes()).unwrap();
    file.sync_all().unwrap();
}
