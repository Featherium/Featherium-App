use std::collections::HashMap;
use std::fmt;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct InstanceId(pub Uuid);

impl InstanceId {
    pub fn new() -> Self {
        InstanceId(Uuid::new_v4())
    }
}

impl fmt::Display for InstanceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct InstanceRecord {
    pub webview_label: String,
    pub recipe_id: String,
    pub label: String,
}

#[derive(Default)]
pub struct InstanceManager(pub Mutex<HashMap<InstanceId, InstanceRecord>>);
