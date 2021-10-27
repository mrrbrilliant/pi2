use chrono::{DateTime, Utc};
use libpm::{Architecture, Dependency, License, Manifest, Package, Source};
use std::{
    collections::HashMap,
    fs::{write, File},
    hash::Hash,
    io::prelude::*,
};
use toml::to_string_pretty;

pub fn gen() {
    let now: DateTime<Utc> = Utc::now();

    let package = Package {
        name: String::from("pi"),
        version: String::from("0.2.0"),
        release: now.timestamp() as usize,
        description: Some(String::from("KOOMPI CLI Package Manager")),
        architecture: vec![Architecture::X86_64],
        license: vec![License::GPL],
        project_url: Some(String::from("https://github.com/koompi/pi")),
        project_owner: Some(String::from("KOOMPI Co., Ltd.")),
        maintainer: Some(vec![
            String::from("Brilliant PHAL <brilliant@koompi.org>"),
            String::from("Hangsia HONG <hangsia@koompi.org>"),
        ]),
    };

    let mut build_map = HashMap::new();
    build_map.insert(String::from("serde"), String::from("*"));
    let mut runtime_map = HashMap::new();
    runtime_map.insert(String::from("tar"), String::from("*"));
    let dependencies = Dependency {
        build: Some(build_map),
        optional: None,
        runtime: Some(runtime_map),
        test: None,
    };

    let mut git_source_map = HashMap::new();
    let mut web_source_map = HashMap::new();
    let mut file_source_map = HashMap::new();
    git_source_map.insert(
        String::from("linux"),
        String::from("https://github.com/linux/linux.git"),
    );
    web_source_map.insert(
        String::from("linux-firmware"),
        String::from("https://kernel.org/release/linux-firmwere.tar.gz"),
    );
    web_source_map.insert(
        String::from("linux-docs"),
        String::from("ftp://kernel.org/release/linux-firmwere.tar.gz"),
    );
    file_source_map.insert(String::from("patch"), String::from("./path"));
    let source = Source {
        web: Some(web_source_map),
        git: Some(git_source_map),
        file: Some(file_source_map),
    };

    let mut envs: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut inter_map: HashMap<String, String> = HashMap::new();
    inter_map.insert("server".to_string(), "123".to_string());
    envs.insert("SECRET".to_string(), inter_map);

    let manifest = Manifest {
        package,
        dependencies: Some(dependencies),
        sources: Some(source),
        environment: Some(envs),
    };

    let data = to_string_pretty(&manifest).unwrap();
    write("pkgbuild.toml", &data).unwrap();
}
