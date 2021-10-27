use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum License {
    Apache2,
    BSD2,
    BSD3,
    GPL,
    LGPL,
    MIT,
    MPL2,
    CDDL,
    EPL2,
    PSF,
    CUSTOM,
}
