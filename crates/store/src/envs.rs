use std::{collections::HashMap, path::PathBuf};

pub fn set_envs() {
    let mut env_map: HashMap<String, String> = HashMap::new();
    insert("PI_DIR_ROOT", "", &mut env_map);
    insert("PI_DIR_LIB", "var/lib/pi", &mut env_map);
    insert("PI_DIR_CACHE", "var/lib/pi/cache", &mut env_map);
    insert("PI_DIR_DATABASE", "var/lib/pi/db", &mut env_map);
    insert("PI_DIR_REGISTRY", "var/lib/pi/registry", &mut env_map);
    insert("PI_DIR_CONFIG", "etc/pi", &mut env_map);
    insert("PI_DIR_TEMP", "tmp/pi", &mut env_map);

    for (k, v) in env_map.iter() {
        std::env::set_var(&k, &v);
    }
}

fn insert(k: &str, v: &str, m: &mut HashMap<String, String>) {
    let root = if cfg!(debug_assertions) {
        std::env::current_dir().unwrap().join("rootfs/")
    } else {
        PathBuf::from("/")
    };
    m.insert(k.to_string(), root.join(v).to_str().unwrap().to_string());
}
