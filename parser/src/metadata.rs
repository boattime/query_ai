use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CodeEntity {
    pub name: String,
    pub kind: String,
    pub details: String,
}
