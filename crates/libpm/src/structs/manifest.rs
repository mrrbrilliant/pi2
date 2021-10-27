use std::{collections::HashMap, path::Path, slice::SliceIndex};

use super::{Dependency, Package, Source};
use crate::enums::SourceType;
use serde::{Deserialize, Serialize};
use std::{
    fs::{canonicalize, copy, create_dir_all, File},
    io::prelude::*,
    process::Command,
};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Manifest {
    pub package: Package,
    pub dependencies: Option<Dependency>,
    pub sources: Option<Source>,
    pub environment: Option<HashMap<String, HashMap<String, String>>>,
}

impl Manifest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn open(path: &str) -> Self {
        let mut buf = String::new();
        let mut file = File::open(path).unwrap();
        file.read_to_string(&mut buf).unwrap();
        let data: Self = toml::from_str(&buf).unwrap();
        data
    }

    pub fn sources(&self) {
        if let Some(sources) = &self.sources {
            let tmp_dir = std::env::var("PI_DIR_TEMP").unwrap();
            let tmp_dir_path = std::path::Path::new(&tmp_dir);
            let pkg_tmp_src_dir_path = &tmp_dir_path.join(&self.package.name).join("src");

            create_dir_all(&pkg_tmp_src_dir_path).unwrap();

            if let Some(web) = &sources.web {
                for (_, v) in web.iter() {
                    Command::new("wget")
                        .args(&[
                            "-qnc",
                            v,
                            "--directory-prefix",
                            pkg_tmp_src_dir_path.to_str().unwrap(),
                        ])
                        .output()
                        .unwrap();
                }
            }

            if let Some(git) = &sources.git {
                for (_, v) in git.iter() {
                    create_dir_all(&pkg_tmp_src_dir_path).unwrap();

                    Command::new("git")
                        .args(&["clone", &v])
                        .current_dir(&pkg_tmp_src_dir_path)
                        .output()
                        .unwrap();
                }
            }

            if let Some(file) = &sources.file {
                for (k, v) in file.iter() {
                    let save_as = pkg_tmp_src_dir_path.join(k);
                    copy(v, save_as).unwrap();
                }
            }
        }
    }
}
