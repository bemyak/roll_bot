use super::*;
use serde::{Deserialize, Serialize};
pub type ArrayOfSpellItemVariant0 = String;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ArrayOfSpellItemVariant1 {
    pub entry: String,
    pub hidden: bool,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ArrayOfSpellItem {
    Variant0(ArrayOfSpellItemVariant0),
    Variant1(ArrayOfSpellItemVariant1),
}
pub type ArrayOfSpell = Vec<ArrayOfSpellItem>;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "abilityGeneric")]
pub struct AbilityGeneric {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<String>>,
    #[doc = " A generic object for storing special data for external use-cases. Keys prefixed with \"rd-\" "]
    #[doc = " should be added as \"data-\" HTML attributes when rendering to HTML."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    pub text: String,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DataCondImmuneTypeVariant1 {
    pub special: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DataCondImmuneTypeVariant2 {
    #[serde(rename = "conditionImmune")]
    pub condition_immune: Vec<DataCondition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "preNote")]
    pub pre_note: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum DataCondImmuneType {
    Variant0(DataCondition),
    Variant1(DataCondImmuneTypeVariant1),
    Variant2(DataCondImmuneTypeVariant2),
}
pub type DataCondImmune = DataCondImmuneType;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DataDamImmunePreNoteVariant1 {
    pub special: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DataDamImmunePreNoteVariant2 {
    pub immune: Vec<DataDamImmune>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "preNote")]
    pub pre_note: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum DataDamImmunePreNote {
    Variant0(DataDamageType),
    Variant1(DataDamImmunePreNoteVariant1),
    Variant2(DataDamImmunePreNoteVariant2),
}
pub type DataDamImmune = DataDamImmunePreNote;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "entry")]
pub struct Entry {
    #[doc = " A generic object for storing special data for external use-cases. Keys prefixed with \"rd-\" "]
    #[doc = " should be added as \"data-\" HTML attributes when rendering to HTML."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "entryAbilityAttackMod")]
pub struct EntryAbilityAttackMod {
    pub attributes: Vec<String>,
    #[doc = " A generic object for storing special data for external use-cases. Keys prefixed with \"rd-\" "]
    #[doc = " should be added as \"data-\" HTML attributes when rendering to HTML."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "entryAbilityDc")]
pub struct EntryAbilityDc {
    pub attributes: Vec<String>,
    #[doc = " A generic object for storing special data for external use-cases. Keys prefixed with \"rd-\" "]
    #[doc = " should be added as \"data-\" HTML attributes when rendering to HTML."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "entryActions")]
pub struct EntryActions {
    pub entries: Vec<EntryJson>,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "entryAttack")]
pub struct EntryAttack {
    #[serde(rename = "attackEntries")]
    pub attack_entries: Vec<EntryJson>,
    #[serde(rename = "attackType")]
    pub attack_type: String,
    #[doc = " A generic object for storing special data for external use-cases. Keys prefixed with \"rd-\" "]
    #[doc = " should be added as \"data-\" HTML attributes when rendering to HTML."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(rename = "hitEntries")]
    pub hit_entries: Vec<EntryJson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "entryBonus")]
pub struct EntryBonus {
    #[doc = " A generic object for storing special data for external use-cases. Keys prefixed with \"rd-\" "]
    #[doc = " should be added as \"data-\" HTML attributes when rendering to HTML."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
    pub value: i64,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "entryBonusSpeed")]
pub struct EntryBonusSpeed {
    #[doc = " A generic object for storing special data for external use-cases. Keys prefixed with \"rd-\" "]
    #[doc = " should be added as \"data-\" HTML attributes when rendering to HTML."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
    pub value: i64,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "entryDataCreature")]
pub struct EntryDataCreature {
    #[serde(rename = "dataCreature")]
    pub data_creature: Creature,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "entryDataItem")]
pub struct EntryDataItem {
    #[serde(rename = "dataItem")]
    pub data_item: Item,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "entryDataObject")]
pub struct EntryDataObject {
    #[serde(rename = "dataObject")]
    pub data_object: Object,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "entryDataSpell")]
pub struct EntryDataSpell {
    #[serde(rename = "dataSpell")]
    pub data_spell: Spell,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum EntryDataTrapHazardDataTrapHazard {
    Variant0(Trap),
    Variant1(Hazard),
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "entryDataTrapHazard")]
pub struct EntryDataTrapHazard {
    #[serde(rename = "dataTrapHazard")]
    pub data_trap_hazard: EntryDataTrapHazardDataTrapHazard,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct EntryDiceItemToRoll {
    pub faces: i64,
    #[doc = " (Warning: unused)"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hideModifier")]
    pub hide_modifier: Option<bool>,
    #[doc = " (Warning: unused)"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifier: Option<i64>,
    pub number: i64,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "entryDice")]
pub struct EntryDice {
    #[doc = " A generic object for storing special data for external use-cases. Keys prefixed with \"rd-\" "]
    #[doc = " should be added as \"data-\" HTML attributes when rendering to HTML."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "toRoll")]
    pub to_roll: Option<Vec<EntryDiceItemToRoll>>,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "entryEntries")]
pub struct EntryEntries {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Vec<String>>,
    #[doc = " A generic object for storing special data for external use-cases. Keys prefixed with \"rd-\" "]
    #[doc = " should be added as \"data-\" HTML attributes when rendering to HTML."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    pub entries: Vec<EntryJson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "entryEntriesInlineEntries")]
pub struct EntryEntriesInlineEntries {
    #[doc = " A generic object for storing special data for external use-cases. Keys prefixed with \"rd-\" "]
    #[doc = " should be added as \"data-\" HTML attributes when rendering to HTML."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    pub entries: Vec<EntryJson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "entryFlowBlock")]
pub struct EntryFlowBlock {
    #[doc = " A generic object for storing special data for external use-cases. Keys prefixed with \"rd-\" "]
    #[doc = " should be added as \"data-\" HTML attributes when rendering to HTML."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<Entry>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "entryFlowchart")]
pub struct EntryFlowchart {
    pub blocks: Vec<Entry>,
    #[doc = " A generic object for storing special data for external use-cases. Keys prefixed with \"rd-\" "]
    #[doc = " should be added as \"data-\" HTML attributes when rendering to HTML."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "entryGallery")]
pub struct EntryGallery {
    #[doc = " A generic object for storing special data for external use-cases. Keys prefixed with \"rd-\" "]
    #[doc = " should be added as \"data-\" HTML attributes when rendering to HTML."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub images: Vec<EntryImage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "entryHomebrew")]
pub struct EntryHomebrew {
    #[doc = " A generic object for storing special data for external use-cases. Keys prefixed with \"rd-\" "]
    #[doc = " should be added as \"data-\" HTML attributes when rendering to HTML."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<EntryJson>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "movedTo")]
    pub moved_to: Option<EntryJson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "oldEntries")]
    pub old_entries: Option<Vec<EntryJson>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "entryHr")]
pub struct EntryHr {
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct EntryImageItemMapRegions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub points: Option<Vec<Vec<i64>>>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "entryImage")]
pub struct EntryImage {
    #[doc = " For accessibility purposes"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "altText")]
    pub alt_text: Option<String>,
    #[doc = " A generic object for storing special data for external use-cases. Keys prefixed with \"rd-\" "]
    #[doc = " should be added as \"data-\" HTML attributes when rendering to HTML."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    pub href: MediaHref,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hrefThumbnail")]
    pub href_thumbnail: Option<MediaHref>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "imageType")]
    pub image_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mapRegions")]
    pub map_regions: Option<Vec<EntryImageItemMapRegions>>,
    #[doc = " As per \"maxWidth\""]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "maxHeight")]
    pub max_height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "maxHeightUnits")]
    pub max_height_units: Option<String>,
    #[doc = " Specify the max desired display width of the images, as opposed to \"width\" which should "]
    #[doc = " only be used for the _real_ width. Assumes pixels by default."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "maxWidth")]
    pub max_width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "maxWidthUnits")]
    pub max_width_units: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "entryIngredient")]
pub struct EntryIngredient {
    #[doc = " A generic object for storing special data for external use-cases. Keys prefixed with \"rd-\" "]
    #[doc = " should be added as \"data-\" HTML attributes when rendering to HTML."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    pub entry: EntryJson,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "entryInlineEntries")]
pub struct EntryInlineEntries {
    #[doc = " A generic object for storing special data for external use-cases. Keys prefixed with \"rd-\" "]
    #[doc = " should be added as \"data-\" HTML attributes when rendering to HTML."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    pub entries: Vec<EntryJson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "entryInset")]
pub struct EntryInset {
    #[doc = " A generic object for storing special data for external use-cases. Keys prefixed with \"rd-\" "]
    #[doc = " should be added as \"data-\" HTML attributes when rendering to HTML."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    pub entries: Vec<EntryJson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "entryInsetReadaloud")]
pub struct EntryInsetReadaloud {
    #[doc = " A generic object for storing special data for external use-cases. Keys prefixed with \"rd-\" "]
    #[doc = " should be added as \"data-\" HTML attributes when rendering to HTML."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    pub entries: Vec<EntryJson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "entryItem")]
pub struct EntryItem {
    #[doc = " A generic object for storing special data for external use-cases. Keys prefixed with \"rd-\" "]
    #[doc = " should be added as \"data-\" HTML attributes when rendering to HTML."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "entryItemSpell")]
pub struct EntryItemSpell {
    #[doc = " A generic object for storing special data for external use-cases. Keys prefixed with \"rd-\" "]
    #[doc = " should be added as \"data-\" HTML attributes when rendering to HTML."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    pub entry: EntryJson,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "entryItemSub")]
pub struct EntryItemSub {
    #[doc = " A generic object for storing special data for external use-cases. Keys prefixed with \"rd-\" "]
    #[doc = " should be added as \"data-\" HTML attributes when rendering to HTML."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    pub entry: EntryJson,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct EntryLinkHrefVariant0Hover {
    #[doc = " Optional; overrides the href hash for hover handlers."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hashPreEncoded")]
    pub hash_pre_encoded: Option<bool>,
    pub page: String,
    pub source: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct EntryLinkHrefVariant0ItemSubhashesVariant0 {
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "preEncoded")]
    pub pre_encoded: Option<bool>,
    pub values: Vec<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct EntryLinkHrefVariant0ItemSubhashesVariant1 {
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "preEncoded")]
    pub pre_encoded: Option<bool>,
    pub value: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum EntryLinkHrefVariant0ItemSubhashes {
    Variant0(EntryLinkHrefVariant0ItemSubhashesVariant0),
    Variant1(EntryLinkHrefVariant0ItemSubhashesVariant1),
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct EntryLinkHrefVariant0 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hashPreEncoded")]
    pub hash_pre_encoded: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover: Option<EntryLinkHrefVariant0Hover>,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subhashes: Option<Vec<EntryLinkHrefVariant0ItemSubhashes>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<serde_json::Value>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct EntryLinkHrefVariant1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<serde_json::Value>,
    pub url: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum EntryLinkHref {
    Variant0(EntryLinkHrefVariant0),
    Variant1(EntryLinkHrefVariant1),
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "entryLink")]
pub struct EntryLink {
    #[doc = " A generic object for storing special data for external use-cases. Keys prefixed with \"rd-\" "]
    #[doc = " should be added as \"data-\" HTML attributes when rendering to HTML."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    pub href: EntryLinkHref,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    pub text: String,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "entryList")]
pub struct EntryList {
    #[doc = " Number of columns the content should be split into. Note that the full value is only "]
    #[doc = " displayed on wide screens, and screens below certain widths will see an appropriately "]
    #[doc = " reduced number of columns."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<i64>,
    #[doc = " A generic object for storing special data for external use-cases. Keys prefixed with \"rd-\" "]
    #[doc = " should be added as \"data-\" HTML attributes when rendering to HTML."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub items: Vec<EntryJson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}
#[doc = " For e.g. Eldritch Invocations which require prerequisite text"]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "entryOptFeature")]
pub struct EntryOptFeature {
    #[doc = " A generic object for storing special data for external use-cases. Keys prefixed with \"rd-\" "]
    #[doc = " should be added as \"data-\" HTML attributes when rendering to HTML."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prerequisite: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "entryOptions")]
pub struct EntryOptions {
    #[doc = " Used to specify how many of the listed options can be chosen as e.g. permanent character "]
    #[doc = " features. Leave blank for transient choices."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = " A generic object for storing special data for external use-cases. Keys prefixed with \"rd-\" "]
    #[doc = " should be added as \"data-\" HTML attributes when rendering to HTML."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    pub entries: Vec<EntryJson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "entryQuote")]
pub struct EntryQuote {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by: Option<String>,
    #[doc = " A generic object for storing special data for external use-cases. Keys prefixed with \"rd-\" "]
    #[doc = " should be added as \"data-\" HTML attributes when rendering to HTML."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    pub entries: Vec<EntryJson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "skipItalics")]
    pub skip_italics: Option<bool>,
    #[doc = " If the automatically-inserted quotation marks should be skipped."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "skipMarks")]
    pub skip_marks: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}
#[doc = " For use in classes page content only."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "entryRefClassFeature")]
pub struct EntryRefClassFeature {
    #[serde(rename = "classFeature")]
    pub class_feature: String,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}
#[doc = " For use in classes page content only."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "entryRefOptionalfeature")]
pub struct EntryRefOptionalfeature {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub optionalfeature: String,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}
#[doc = " For use in classes page content only."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "entryRefSubclassFeature")]
pub struct EntryRefSubclassFeature {
    #[serde(rename = "subclassFeature")]
    pub subclass_feature: String,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "entrySection")]
pub struct EntrySection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Vec<String>>,
    #[doc = " A generic object for storing special data for external use-cases. Keys prefixed with \"rd-\" "]
    #[doc = " should be added as \"data-\" HTML attributes when rendering to HTML."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    pub entries: Vec<EntryJson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct EntrySpellcastingSpells0 {
    pub spells: Vec<String>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct EntrySpellcastingSpells {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "0")]
    pub _0: Option<EntrySpellcastingSpells0>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "1")]
    pub _1: Option<EntrySpellcastingLevel1To9>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "2")]
    pub _2: Option<EntrySpellcastingLevel1To9>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "3")]
    pub _3: Option<EntrySpellcastingLevel1To9>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "4")]
    pub _4: Option<EntrySpellcastingLevel1To9>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "5")]
    pub _5: Option<EntrySpellcastingLevel1To9>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "6")]
    pub _6: Option<EntrySpellcastingLevel1To9>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "7")]
    pub _7: Option<EntrySpellcastingLevel1To9>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "8")]
    pub _8: Option<EntrySpellcastingLevel1To9>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "9")]
    pub _9: Option<EntrySpellcastingLevel1To9>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "entrySpellcasting")]
pub struct EntrySpellcasting {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ability: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant: Option<ArrayOfSpell>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily: Option<EntrySpellcastingFrequency>,
    #[doc = " A generic object for storing special data for external use-cases. Keys prefixed with \"rd-\" "]
    #[doc = " should be added as \"data-\" HTML attributes when rendering to HTML."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    #[doc = " Implicitly \"trait\""]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "displayAs")]
    pub display_as: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "footerEntries")]
    pub footer_entries: Option<Vec<EntryJson>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "headerEntries")]
    pub header_entries: Option<Vec<EntryJson>>,
    #[doc = " Allows the above properties to be specified, but not rendered. Useful if e.g. a creature "]
    #[doc = " can only cast one spell, and this is rendered in the header line."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rest: Option<EntrySpellcastingFrequency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ritual: Option<ArrayOfSpell>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spells: Option<EntrySpellcastingSpells>,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly: Option<EntrySpellcastingFrequency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub will: Option<ArrayOfSpell>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(rename = "entrySpellcasting_frequency")]
pub struct EntrySpellcastingFrequency {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "1")]
    pub _1: Option<ArrayOfSpell>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "1e")]
    pub _1e: Option<ArrayOfSpell>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "2")]
    pub _2: Option<ArrayOfSpell>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "2e")]
    pub _2e: Option<ArrayOfSpell>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "3")]
    pub _3: Option<ArrayOfSpell>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "3e")]
    pub _3e: Option<ArrayOfSpell>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "4")]
    pub _4: Option<ArrayOfSpell>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "4e")]
    pub _4e: Option<ArrayOfSpell>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "5")]
    pub _5: Option<ArrayOfSpell>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "5e")]
    pub _5e: Option<ArrayOfSpell>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "6")]
    pub _6: Option<ArrayOfSpell>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "6e")]
    pub _6e: Option<ArrayOfSpell>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "7")]
    pub _7: Option<ArrayOfSpell>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "7e")]
    pub _7e: Option<ArrayOfSpell>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "8")]
    pub _8: Option<ArrayOfSpell>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "8e")]
    pub _8e: Option<ArrayOfSpell>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "9")]
    pub _9: Option<ArrayOfSpell>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "9e")]
    pub _9e: Option<ArrayOfSpell>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "entrySpellcasting_level1to9")]
pub struct EntrySpellcastingLevel1To9 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slots: Option<f64>,
    pub spells: Vec<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "entryTable")]
pub struct EntryTable {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "colLabels")]
    pub col_labels: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "colStyles")]
    pub col_styles: Option<Vec<String>>,
    #[doc = " A generic object for storing special data for external use-cases. Keys prefixed with \"rd-\" "]
    #[doc = " should be added as \"data-\" HTML attributes when rendering to HTML."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footnotes: Option<Vec<EntryJson>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = " Primarily for homebrew use."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intro: Option<Vec<EntryJson>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "isNameGenerator")]
    pub is_name_generator: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "isStriped")]
    pub is_striped: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = " Primarily for homebrew use."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outro: Option<Vec<EntryJson>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "rowLabels")]
    pub row_labels: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "rowStyles")]
    pub row_styles: Option<Vec<String>>,
    pub rows: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct EntryTableCellRollVariant0 {
    pub max: i64,
    pub min: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pad: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct EntryTableCellRollVariant1 {
    pub exact: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pad: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum EntryTableCellRoll {
    Variant0(EntryTableCellRollVariant0),
    Variant1(EntryTableCellRollVariant1),
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "entryTableCell")]
pub struct EntryTableCell {
    #[doc = " A generic object for storing special data for external use-cases. Keys prefixed with \"rd-\" "]
    #[doc = " should be added as \"data-\" HTML attributes when rendering to HTML."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry: Option<EntryJson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roll: Option<EntryTableCellRoll>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
}
#[doc = " Used to group related tables together; has no effect on rendering."]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "entryTableGroup")]
pub struct EntryTableGroup {
    #[doc = " A generic object for storing special data for external use-cases. Keys prefixed with \"rd-\" "]
    #[doc = " should be added as \"data-\" HTML attributes when rendering to HTML."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables: Option<Vec<EntryTable>>,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "entryTableRow")]
pub struct EntryTableRow {
    #[doc = " A generic object for storing special data for external use-cases. Keys prefixed with \"rd-\" "]
    #[doc = " should be added as \"data-\" HTML attributes when rendering to HTML."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    pub row: Vec<EntryJson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct EntryVariantVariantSource {
    pub page: i64,
    pub source: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "entryVariant")]
pub struct EntryVariant {
    #[doc = " A generic object for storing special data for external use-cases. Keys prefixed with \"rd-\" "]
    #[doc = " should be added as \"data-\" HTML attributes when rendering to HTML."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    pub entries: Vec<EntryJson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "variantSource")]
    pub variant_source: Option<EntryVariantVariantSource>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "entryVariantInner")]
pub struct EntryVariantInner {
    #[doc = " A generic object for storing special data for external use-cases. Keys prefixed with \"rd-\" "]
    #[doc = " should be added as \"data-\" HTML attributes when rendering to HTML."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    pub entries: Vec<EntryJson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "entryVariantSub")]
pub struct EntryVariantSub {
    #[doc = " A generic object for storing special data for external use-cases. Keys prefixed with \"rd-\" "]
    #[doc = " should be added as \"data-\" HTML attributes when rendering to HTML."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    pub entries: Vec<EntryJson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "entryWrapped")]
pub struct EntryWrapped {
    #[doc = " A generic object for storing special data for external use-cases. Keys prefixed with \"rd-\" "]
    #[doc = " should be added as \"data-\" HTML attributes when rendering to HTML."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
    pub wrapped: serde_json::Value,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct MediaHrefWrappedVariant0 {
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<serde_json::Value>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct MediaHrefWrappedVariant1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<serde_json::Value>,
    pub url: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum MediaHrefWrapped {
    Variant0(MediaHrefWrappedVariant0),
    Variant1(MediaHrefWrappedVariant1),
}
pub type MediaHref = MediaHrefWrapped;
pub type EntryJson = Entry;
