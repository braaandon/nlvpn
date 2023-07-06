use serde::{Deserialize, Serialize};

pub struct Port {
    pub(crate) start: u16,
    pub(crate) end: u16,
}

#[derive(Serialize, Deserialize)]
pub enum LimitType {
    Data,
    Instance,
    Members,
    Character,
}

impl LimitType {
    pub fn to_port(&self) -> Port {
        match self {
            LimitType::Data => Port {
                start: 3074,
                end: 3074,
            },
            LimitType::Instance => Port {
                start: 30000,
                end: 30009,
            },
            LimitType::Members => Port {
                start: 27015,
                end: 27200,
            },
            LimitType::Character => Port {
                start: 7500,
                end: 7509,
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct LimitReq {
    pub(crate) state: bool,
    pub(crate) limit: LimitType,
}
