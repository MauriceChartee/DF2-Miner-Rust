use std::collections::HashMap;
use std::fs;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OcelLog {
    #[serde(rename = "ocel:global-event")]
    pub global_event: GlobalEvent,

    #[serde(rename = "ocel:global-object")]
    pub global_object: GlobalObject,

    #[serde(rename = "ocel:global-log")]
    pub global_log: GlobalLog,

    #[serde(rename = "ocel:events")]
    pub events: HashMap<String, OcelEvent>,
}

#[derive(Debug, Deserialize)]
pub struct GlobalEvent {
    #[serde(rename = "ocel:activity")]
    pub activity: String,
}

#[derive(Debug, Deserialize)]
pub struct GlobalObject {
    #[serde(rename = "ocel:type")]
    pub object_type: String,
}

#[derive(Debug, Deserialize)]
pub struct GlobalLog {
    #[serde(rename = "ocel:attribute-names")]
    pub attribute_names: Vec<String>,

    #[serde(rename = "ocel:object-types")]
    pub object_types: Vec<String>,

    #[serde(rename = "ocel:version")]
    pub version: String,

    #[serde(rename = "ocel:ordering")]
    pub ordering: String,
}

#[derive(Debug, Deserialize)]
pub struct OcelEvent {
    #[serde(rename = "ocel:activity")]
    pub activity: String,

    #[serde(rename = "ocel:timestamp")]
    pub timestamp: String,

    #[serde(rename = "ocel:omap")]
    pub omap: Vec<String>,

    #[serde(rename = "ocel:vmap")]
    pub vmap: HashMap<String, serde_json::Value>,
}

pub fn load_ocel(path: &str) -> Result<OcelLog, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let log: OcelLog = serde_json::from_str(&content)?;
    Ok(log)
}
