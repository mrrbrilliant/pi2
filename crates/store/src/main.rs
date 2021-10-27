pub mod envs;
pub mod generate;

use chrono::{DateTime, Utc};
use libpm::{Architecture, Dependency, License, Manifest, Operation, Package, Source, SourceType};
use std::{
    collections::HashMap,
    ffi::OsString,
    fs::{create_dir_all, write, File},
    hash::Hash,
    io::prelude::*,
    path::Path,
};
use toml::to_string_pretty;

fn prepare() {
    // 1. Environment Variables
    envs::set_envs();
    for (k, v) in std::env::vars() {
        // create directories if not exists
        if k.contains("PI_DIR") {
            if !Path::new(&v).exists() {
                create_dir_all(&v).unwrap();
            }
        }
    }
}

fn cli_parser() -> Vec<HashMap<String, Operation>> {
    let arguments: String = std::env::args().skip(1).collect::<Vec<String>>().join(" ");
    let operation_string: Vec<&str> = arguments.split(",").collect();
    let mut operations: Vec<HashMap<String, Operation>> = Vec::new();

    for op in operation_string.iter() {
        let trim_start = op.trim_start();
        let trim_end = trim_start.trim_end();

        let mut flags: Vec<String> = Vec::new();
        let mut packages: Vec<String> = Vec::new();
        let words: Vec<String> = trim_end.split_whitespace().map(|f| f.to_string()).collect();
        if !words.is_empty() {
            let op_name: String = words.first().unwrap().to_string();

            for v in words.iter().skip(1) {
                if v.starts_with("-") {
                    flags.push(v.to_string());
                } else {
                    packages.push(v.to_string())
                }
            }

            let opr = Operation {
                flags: Some(flags),
                packages: Some(packages),
            };

            let mut map = HashMap::new();

            map.insert(op_name, opr);
            operations.push(map);
        }
    }

    operations
}

fn main() {
    // 1. prepare maindatory configuration
    prepare();
    let operations = cli_parser();

    for op in operations.iter() {
        for (k, v) in op.iter() {
            match k.as_ref() {
                "generate" | "--generate" | "g" | "-g" => {
                    // println!("Publishing: {:?}", v);
                    generate::gen();
                }
                "install" | "--install" | "i" | "-i" => {
                    println!("Installing: {:?}", v);
                }
                "remove" | "--remove" | "r" | "-r" => {
                    println!("Removing: {:?}", v);
                }
                "update" | "--update" | "u" | "-u" => {
                    println!("Updating");
                }
                "build" | "--build" | "b" | "-b" => {
                    if let Some(packages) = &v.packages {
                        if !packages.is_empty() {
                            for pkg in packages.iter() {
                                let package = Manifest::open(pkg);
                                println!("{:#?}", package);
                                package.sources();
                            }
                        } else {
                            println!("Build use pkgbuild.toml");
                        }
                    }
                }
                "publish" | "--publish" | "p" | "-p" => {
                    println!("Publishing: {:?}", v);
                }
                _ => {
                    println!("{} is invalid operation name", k);
                }
            }
        }
    }
}
