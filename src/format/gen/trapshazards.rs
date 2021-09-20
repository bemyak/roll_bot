use super::*;
use serde::{Deserialize, Serialize};
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "hazard")]
pub struct Hazard {
    pub entries: Vec<EntryJson>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srd: Option<Srd>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "trapHazType")]
    pub trap_haz_type: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct TrapTrapHazTypeVariant0 {
    pub entries: Vec<EntryJson>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srd: Option<Srd>,
    #[serde(rename = "trapHazType")]
    pub trap_haz_type: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct TrapTrapHazTypeVariant1 {
    pub countermeasures: Vec<EntryJson>,
    pub effect: Vec<EntryJson>,
    pub entries: Vec<EntryJson>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srd: Option<Srd>,
    pub threat: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<i64>,
    #[serde(rename = "trapHazType")]
    pub trap_haz_type: serde_json::Value,
    pub trigger: Vec<EntryJson>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct TrapTrapHazTypeVariant2 {
    pub countermeasures: Vec<EntryJson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "eActive")]
    pub e_active: Option<Vec<EntryJson>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "eConstant")]
    pub e_constant: Option<Vec<EntryJson>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "eDynamic")]
    pub e_dynamic: Option<Vec<EntryJson>>,
    pub entries: Vec<EntryJson>,
    pub initiative: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "initiativeNote")]
    pub initiative_note: Option<EntryJson>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srd: Option<Srd>,
    pub threat: i64,
    pub tier: i64,
    #[serde(rename = "trapHazType")]
    pub trap_haz_type: serde_json::Value,
    pub trigger: Vec<EntryJson>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum TrapTrapHazType {
    Variant0(TrapTrapHazTypeVariant0),
    Variant1(TrapTrapHazTypeVariant1),
    Variant2(TrapTrapHazTypeVariant2),
}
pub type Trap = TrapTrapHazType;
