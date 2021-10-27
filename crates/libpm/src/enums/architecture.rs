use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Architecture {
    Aarch64,
    Armhf,
    RiscV32,
    RiscV64,
    X86,
    X86_64,
}
