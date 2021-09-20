use super::*;
use serde::{Deserialize, Serialize};
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "class")]
pub struct Class {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "definedInSource")]
    pub defined_in_source: Option<String>,
    pub name: String,
    pub source: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DurationDuration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "upTo")]
    pub up_to: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "duration")]
pub struct Duration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concentration: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<DurationDuration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ends: Option<Vec<String>>,
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct ScalingLevelDiceItemScaling {}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(rename = "scalingLevelDiceItem")]
pub struct ScalingLevelDiceItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling: Option<ScalingLevelDiceItemScaling>,
}
pub type Spell = serde_json::Value;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SpellDataItemBackgrounds {
    pub name: String,
    pub source: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SpellDataClassesItemFromSubclassSubclass {
    pub name: String,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "subSubclass")]
    pub sub_subclass: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SpellDataClassesItemFromSubclass {
    pub class: Class,
    pub subclass: SpellDataClassesItemFromSubclassSubclass,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct SpellDataClasses {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fromClassList")]
    pub from_class_list: Option<Vec<Class>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fromClassListVariant")]
    pub from_class_list_variant: Option<Vec<Class>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fromSubclass")]
    pub from_subclass: Option<Vec<SpellDataClassesItemFromSubclass>>,
}
pub type SpellDataComponentsMVariant0ConsumeVariant0 = bool;
pub type SpellDataComponentsMVariant0ConsumeVariant1 = String;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SpellDataComponentsMVariant0Consume {
    Variant0(SpellDataComponentsMVariant0ConsumeVariant0),
    Variant1(SpellDataComponentsMVariant0ConsumeVariant1),
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SpellDataComponentsMVariant0 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consume: Option<SpellDataComponentsMVariant0Consume>,
    #[doc = " In copper pieces."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<f64>,
    pub text: String,
}
pub type SpellDataComponentsMVariant1 = serde_json::Value;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SpellDataComponentsM {
    Variant0(SpellDataComponentsMVariant0),
    Variant1(SpellDataComponentsMVariant1),
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct SpellDataComponents {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m: Option<SpellDataComponentsM>,
    #[doc = " \"Royalty\" components, as introduced in Acquisitions Incorporated"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SpellDataItemEldritchInvocations {
    pub name: String,
    pub source: String,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct SpellDataMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ritual: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub technomagic: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SpellDataItemRaces {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "baseName")]
    pub base_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "baseSource")]
    pub base_source: Option<String>,
    pub name: String,
    pub source: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SpellDataRangeDistance {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[doc = " Homebrew only"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "amountSecondary")]
    pub amount_secondary: Option<i64>,
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = " Homebrew only"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "typeSecondary")]
    pub type_secondary: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SpellDataRange {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance: Option<SpellDataRangeDistance>,
    #[serde(rename = "type")]
    pub type_: String,
}
pub type SpellDataScalingLevelDiceVariant0 = Vec<ScalingLevelDiceItem>;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SpellDataScalingLevelDice {
    Variant0(SpellDataScalingLevelDiceVariant0),
    Variant1(ScalingLevelDiceItem),
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(rename = "spellData")]
pub struct SpellData {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "abilityCheck")]
    pub ability_check: Option<Vec<String>>,
    #[doc = " By convention, only the effects of the spell cast at its base level are considered when "]
    #[doc = " populating these."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "areaTags")]
    pub area_tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backgrounds: Option<Vec<SpellDataItemBackgrounds>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classes: Option<SpellDataClasses>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<SpellDataComponents>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "conditionInflict")]
    pub condition_inflict: Option<TagsConditions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "damageImmune")]
    pub damage_immune: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "damageInflict")]
    pub damage_inflict: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "damageResist")]
    pub damage_resist: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "damageVulnerable")]
    pub damage_vulnerable: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Vec<Duration>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "eldritchInvocations")]
    pub eldritch_invocations: Option<Vec<SpellDataItemEldritchInvocations>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<EntryJson>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "entriesHigherLevel")]
    pub entries_higher_level: Option<Vec<EntryJson>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hasFluff")]
    pub has_fluff: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hasFluffImages")]
    pub has_fluff_images: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<SpellDataMeta>,
    #[doc = " PRM = permanentEffects = 'Used to flag spells that can/will have permanent effects, but are "]
    #[doc = " not listed as having permanent duration'; SCL = scalingEffects; HL = isHeal; SMN = "]
    #[doc = " isSummon; SGT = requiresSight; THP = isTempHp; MAC = isModifiesAc; TP = isTeleport; FMV = "]
    #[doc = " isForcedMovement"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "miscTags")]
    pub misc_tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "otherSources")]
    pub other_sources: Option<OtherSources>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub races: Option<Vec<SpellDataItemRaces>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<SpellDataRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "savingThrow")]
    pub saving_throw: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "scalingLevelDice")]
    pub scaling_level_dice: Option<SpellDataScalingLevelDice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub school: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "spellAttack")]
    pub spell_attack: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srd: Option<Srd>,
    #[doc = " For homebrew use only."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subschools: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<Vec<Time>>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "time")]
pub struct Time {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,
    pub unit: String,
}
