use super::*;
use serde::{Deserialize, Serialize};
pub type ObjectAcVariant0 = i64;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ObjectAcVariant1 {
    pub special: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ObjectAc {
    Variant0(ObjectAcVariant0),
    Variant1(ObjectAcVariant1),
}
pub type ObjectHpVariant0 = i64;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ObjectHpVariant1 {
    pub special: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ObjectHp {
    Variant0(ObjectHpVariant0),
    Variant1(ObjectHpVariant1),
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "object")]
pub struct Object {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ac: Option<ObjectAc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "actionEntries")]
    pub action_entries: Option<Vec<EntryJson>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cha: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub con: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "conditionImmune")]
    pub condition_immune: Option<ConditionImmunityArray>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "creatureType")]
    pub creature_type: Option<CreatureType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dex: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<EntryJson>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hasToken")]
    pub has_token: Option<bool>,
    pub hp: ObjectHp,
    pub immune: DamageImmunityArray,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub int: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "isNpc")]
    pub is_npc: Option<bool>,
    pub name: String,
    #[doc = " siege weapon; generic; unknown"]
    #[serde(rename = "objectType")]
    pub object_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resist: Option<DamageResistArray>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub senses: Option<serde_json::Value>,
    pub size: String,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<Speed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srd: Option<Srd>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub str: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tokenUrl")]
    pub token_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulnerable: Option<DamageVulnerabilityArray>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wis: Option<i64>,
}
