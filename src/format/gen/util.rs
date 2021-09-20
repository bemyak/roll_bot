use super::*;
use serde::{Deserialize, Serialize};
pub type AdditionalSpellArrayOfStringOrChoiceObjectItemVariant0 = String;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct AdditionalSpellArrayOfStringOrChoiceObjectItemVariant1 {
    pub choose: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AdditionalSpellArrayOfStringOrChoiceObjectItem {
    Variant0(AdditionalSpellArrayOfStringOrChoiceObjectItemVariant0),
    Variant1(AdditionalSpellArrayOfStringOrChoiceObjectItemVariant1),
}
pub type AdditionalSpellArrayOfStringOrChoiceObject =
    Vec<AdditionalSpellArrayOfStringOrChoiceObjectItem>;
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(rename = "_additionalSpellLevelObject")]
pub struct AdditionalSpellLevelObject {
    #[doc = " Spells which do not fall into the above categories (i.e. have no specific recharge, and are "]
    #[doc = " simply known)"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_")]
    pub underscore_: Option<AdditionalSpellArrayOfStringOrChoiceObject>,
    #[doc = " Spells which recharge on long rest"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily: Option<AdditionalSpellRechargeObject>,
    #[doc = " Spells which cost a specific resource to use (such as Ki), but otherwise have no "]
    #[doc = " restrictions"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<AdditionalSpellRechargeObject>,
    #[doc = " Spells which recharge on short or long rest"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rest: Option<AdditionalSpellRechargeObject>,
    #[doc = " Ritual-only spells"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ritual: Option<AdditionalSpellArrayOfStringOrChoiceObject>,
    #[doc = " At-will spells"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub will: Option<AdditionalSpellArrayOfStringOrChoiceObject>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AdditionalSpellObjectUnderscore {
    Variant0(AdditionalSpellArrayOfStringOrChoiceObject),
    Variant1(AdditionalSpellLevelObject),
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(rename = "_additionalSpellObject")]
pub struct AdditionalSpellObject {
    #[doc = " Spells gained when gaining a particular feature, regardless of level or caster level"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_")]
    pub underscore_: Option<AdditionalSpellObjectUnderscore>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(rename = "_additionalSpellRechargeObject")]
pub struct AdditionalSpellRechargeObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "1")]
    pub _1: Option<AdditionalSpellArrayOfStringOrChoiceObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "1e")]
    pub _1e: Option<AdditionalSpellArrayOfStringOrChoiceObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "2")]
    pub _2: Option<AdditionalSpellArrayOfStringOrChoiceObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "2e")]
    pub _2e: Option<AdditionalSpellArrayOfStringOrChoiceObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "3")]
    pub _3: Option<AdditionalSpellArrayOfStringOrChoiceObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "3e")]
    pub _3e: Option<AdditionalSpellArrayOfStringOrChoiceObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "4")]
    pub _4: Option<AdditionalSpellArrayOfStringOrChoiceObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "4e")]
    pub _4e: Option<AdditionalSpellArrayOfStringOrChoiceObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "5")]
    pub _5: Option<AdditionalSpellArrayOfStringOrChoiceObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "5e")]
    pub _5e: Option<AdditionalSpellArrayOfStringOrChoiceObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "6")]
    pub _6: Option<AdditionalSpellArrayOfStringOrChoiceObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "6e")]
    pub _6e: Option<AdditionalSpellArrayOfStringOrChoiceObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "7")]
    pub _7: Option<AdditionalSpellArrayOfStringOrChoiceObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "7e")]
    pub _7e: Option<AdditionalSpellArrayOfStringOrChoiceObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "8")]
    pub _8: Option<AdditionalSpellArrayOfStringOrChoiceObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "8e")]
    pub _8e: Option<AdditionalSpellArrayOfStringOrChoiceObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "9")]
    pub _9: Option<AdditionalSpellArrayOfStringOrChoiceObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "9e")]
    pub _9e: Option<AdditionalSpellArrayOfStringOrChoiceObject>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SpeedVal9EVariant0 {
    pub condition: String,
    pub number: i64,
}
pub type SpeedVal9EVariant1 = i64;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SpeedVal9E {
    Variant0(SpeedVal9EVariant0),
    Variant1(SpeedVal9EVariant1),
}
pub type SpeedVal = SpeedVal9E;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct AdditionalSourcesItemNumber {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    pub source: String,
}
pub type AdditionalSources = Vec<AdditionalSourcesItemNumber>;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct AdditionalSpellsArrayItemSourceAbilityVariant0 {
    pub choose: Vec<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum AdditionalSpellsArrayItemSourceAbilityVariant1 {
    #[serde(rename = "str")]
    Str,
    #[serde(rename = "dex")]
    Dex,
    #[serde(rename = "con")]
    Con,
    #[serde(rename = "int")]
    Int,
    #[serde(rename = "wis")]
    Wis,
    #[serde(rename = "cha")]
    Cha,
    #[serde(rename = "inherit")]
    Inherit,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AdditionalSpellsArrayItemSourceAbility {
    Variant0(AdditionalSpellsArrayItemSourceAbilityVariant0),
    Variant1(AdditionalSpellsArrayItemSourceAbilityVariant1),
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct AdditionalSpellsArrayItemSource {
    #[doc = " Optionally specify the ability score used for e.g. racial spellcasting"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ability: Option<AdditionalSpellsArrayItemSourceAbility>,
    #[doc = " Expansions to a class' default spell list, from which spells can be chosen (e.g. Warlock "]
    #[doc = " Patron spells)"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expanded: Option<AdditionalSpellObject>,
    #[doc = " Spells which can be innately cast, without expending normal spell resources"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub innate: Option<AdditionalSpellObject>,
    #[doc = " Optional display name for the group"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = " Spells which are always prepared"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepared: Option<AdditionalSpellObject>,
    #[doc = " Optional resource name for resource-cast spells in this group"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resourceName")]
    pub resource_name: Option<String>,
}
#[doc = " A collection of additional spells which a feature grants."]
pub type AdditionalSpellsArray = Vec<AdditionalSpellsArrayItemSource>;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "alignment")]
pub enum Alignment {
    L,
    N,
    #[serde(rename = "NX")]
    Nx,
    #[serde(rename = "NY")]
    Ny,
    C,
    G,
    E,
    U,
    A,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct ArmorProficienciesItemResourceName {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heavy: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "shield|phb")]
    pub shield_phb: Option<bool>,
}
pub type ArmorProficiencies = Vec<ArmorProficienciesItemResourceName>;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ConditionImmunityArrayShieldPhbVariant0ItemShieldPhbVariant1 {
    pub special: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ConditionImmunityArrayShieldPhbVariant0ItemShieldPhbVariant2 {
    #[serde(rename = "conditionImmune")]
    pub condition_immune: ConditionImmunityArray,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "preNote")]
    pub pre_note: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ConditionImmunityArrayShieldPhbVariant0ItemShieldPhb {
    Variant0(DataCondition),
    Variant1(ConditionImmunityArrayShieldPhbVariant0ItemShieldPhbVariant1),
    Variant2(ConditionImmunityArrayShieldPhbVariant0ItemShieldPhbVariant2),
}
pub type ConditionImmunityArrayShieldPhbVariant0 =
    Vec<ConditionImmunityArrayShieldPhbVariant0ItemShieldPhb>;
pub type ConditionImmunityArrayShieldPhbVariant1 = serde_json::Value;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ConditionImmunityArrayShieldPhb {
    Variant0(ConditionImmunityArrayShieldPhbVariant0),
    Variant1(ConditionImmunityArrayShieldPhbVariant1),
}
pub type ConditionImmunityArray = ConditionImmunityArrayShieldPhb;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ConditionImmunityArrayPlayerItemPreNoteVariant1Choose {
    pub from: ConditionImmunityArrayPlayer,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct ConditionImmunityArrayPlayerItemPreNoteVariant1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choose: Option<ConditionImmunityArrayPlayerItemPreNoteVariant1Choose>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ConditionImmunityArrayPlayerItemPreNote {
    Variant0(DataCondition),
    Variant1(ConditionImmunityArrayPlayerItemPreNoteVariant1),
}
pub type ConditionImmunityArrayPlayer = Vec<ConditionImmunityArrayPlayerItemPreNote>;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CopyBlockCopyTrait {
    pub name: String,
    pub source: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CopyBlockCopy {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_mod")]
    pub mod_: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_preserve")]
    pub preserve: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_trait")]
    pub trait_: Option<CopyBlockCopyTrait>,
    #[doc = " Used in subclass data"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "className")]
    pub class_name: Option<String>,
    #[doc = " Used in subclass data"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "classSource")]
    pub class_source: Option<String>,
    pub name: String,
    #[doc = " Used in deity data"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pantheon: Option<String>,
    #[doc = " Used in subclass data"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "shortName")]
    pub short_name: Option<String>,
    pub source: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "copyBlock")]
pub struct CopyBlock {
    #[serde(rename = "_copy")]
    pub copy: CopyBlockCopy,
}
pub type CopyModifier = serde_json::Value;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "creatureType")]
pub enum CreatureType {
    #[serde(rename = "aberration")]
    Aberration,
    #[serde(rename = "beast")]
    Beast,
    #[serde(rename = "celestial")]
    Celestial,
    #[serde(rename = "construct")]
    Construct,
    #[serde(rename = "dragon")]
    Dragon,
    #[serde(rename = "elemental")]
    Elemental,
    #[serde(rename = "fey")]
    Fey,
    #[serde(rename = "fiend")]
    Fiend,
    #[serde(rename = "giant")]
    Giant,
    #[serde(rename = "humanoid")]
    Humanoid,
    #[serde(rename = "monstrosity")]
    Monstrosity,
    #[serde(rename = "ooze")]
    Ooze,
    #[serde(rename = "plant")]
    Plant,
    #[serde(rename = "undead")]
    Undead,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DamageImmunityArraySourceVariant0ItemSourceVariant1 {
    pub special: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DamageImmunityArraySourceVariant0ItemSourceVariant2 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cond: Option<bool>,
    pub immune: DamageImmunityArray,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "preNote")]
    pub pre_note: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum DamageImmunityArraySourceVariant0ItemSource {
    Variant0(DataDamageType),
    Variant1(DamageImmunityArraySourceVariant0ItemSourceVariant1),
    Variant2(DamageImmunityArraySourceVariant0ItemSourceVariant2),
}
pub type DamageImmunityArraySourceVariant0 = Vec<DamageImmunityArraySourceVariant0ItemSource>;
pub type DamageImmunityArraySourceVariant1 = serde_json::Value;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum DamageImmunityArraySource {
    Variant0(DamageImmunityArraySourceVariant0),
    Variant1(DamageImmunityArraySourceVariant1),
}
pub type DamageImmunityArray = DamageImmunityArraySource;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DamageImmunityArrayPlayerItemPreNoteVariant1Choose {
    pub from: DamageImmunityArrayPlayer,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct DamageImmunityArrayPlayerItemPreNoteVariant1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choose: Option<DamageImmunityArrayPlayerItemPreNoteVariant1Choose>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum DamageImmunityArrayPlayerItemPreNote {
    Variant0(DataDamageType),
    Variant1(DamageImmunityArrayPlayerItemPreNoteVariant1),
}
pub type DamageImmunityArrayPlayer = Vec<DamageImmunityArrayPlayerItemPreNote>;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DamageResistArrayFromVariant0ItemFromVariant1 {
    pub special: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DamageResistArrayFromVariant0ItemFromVariant2 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cond: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "preNote")]
    pub pre_note: Option<String>,
    pub resist: DamageResistArray,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum DamageResistArrayFromVariant0ItemFrom {
    Variant0(DataDamageType),
    Variant1(DamageResistArrayFromVariant0ItemFromVariant1),
    Variant2(DamageResistArrayFromVariant0ItemFromVariant2),
}
pub type DamageResistArrayFromVariant0 = Vec<DamageResistArrayFromVariant0ItemFrom>;
pub type DamageResistArrayFromVariant1 = serde_json::Value;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum DamageResistArrayFrom {
    Variant0(DamageResistArrayFromVariant0),
    Variant1(DamageResistArrayFromVariant1),
}
pub type DamageResistArray = DamageResistArrayFrom;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DamageResistArrayPlayerItemResistVariant1Choose {
    pub from: DamageResistArrayPlayer,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct DamageResistArrayPlayerItemResistVariant1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choose: Option<DamageResistArrayPlayerItemResistVariant1Choose>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum DamageResistArrayPlayerItemResist {
    Variant0(DataDamageType),
    Variant1(DamageResistArrayPlayerItemResistVariant1),
}
pub type DamageResistArrayPlayer = Vec<DamageResistArrayPlayerItemResist>;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DamageVulnerabilityArrayFromVariant0ItemFromVariant1 {
    pub special: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DamageVulnerabilityArrayFromVariant0ItemFromVariant2 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cond: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "preNote")]
    pub pre_note: Option<String>,
    pub vulnerable: DamageVulnerabilityArray,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum DamageVulnerabilityArrayFromVariant0ItemFrom {
    Variant0(DataDamageType),
    Variant1(DamageVulnerabilityArrayFromVariant0ItemFromVariant1),
    Variant2(DamageVulnerabilityArrayFromVariant0ItemFromVariant2),
}
pub type DamageVulnerabilityArrayFromVariant0 = Vec<DamageVulnerabilityArrayFromVariant0ItemFrom>;
pub type DamageVulnerabilityArrayFromVariant1 = serde_json::Value;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum DamageVulnerabilityArrayFrom {
    Variant0(DamageVulnerabilityArrayFromVariant0),
    Variant1(DamageVulnerabilityArrayFromVariant1),
}
pub type DamageVulnerabilityArray = DamageVulnerabilityArrayFrom;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DamageVulnerabilityArrayPlayerItemVulnerableVariant1Choose {
    pub from: DamageVulnerabilityArrayPlayer,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct DamageVulnerabilityArrayPlayerItemVulnerableVariant1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choose: Option<DamageVulnerabilityArrayPlayerItemVulnerableVariant1Choose>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum DamageVulnerabilityArrayPlayerItemVulnerable {
    Variant0(DataDamageType),
    Variant1(DamageVulnerabilityArrayPlayerItemVulnerableVariant1),
}
pub type DamageVulnerabilityArrayPlayer = Vec<DamageVulnerabilityArrayPlayerItemVulnerable>;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "dataCondition")]
pub enum DataCondition {
    #[serde(rename = "blinded")]
    Blinded,
    #[serde(rename = "charmed")]
    Charmed,
    #[serde(rename = "deafened")]
    Deafened,
    #[serde(rename = "exhaustion")]
    Exhaustion,
    #[serde(rename = "frightened")]
    Frightened,
    #[serde(rename = "grappled")]
    Grappled,
    #[serde(rename = "incapacitated")]
    Incapacitated,
    #[serde(rename = "invisible")]
    Invisible,
    #[serde(rename = "paralyzed")]
    Paralyzed,
    #[serde(rename = "petrified")]
    Petrified,
    #[serde(rename = "poisoned")]
    Poisoned,
    #[serde(rename = "prone")]
    Prone,
    #[serde(rename = "restrained")]
    Restrained,
    #[serde(rename = "stunned")]
    Stunned,
    #[serde(rename = "unconscious")]
    Unconscious,
    #[serde(rename = "disease")]
    Disease,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "dataDamageType")]
pub enum DataDamageType {
    #[serde(rename = "acid")]
    Acid,
    #[serde(rename = "bludgeoning")]
    Bludgeoning,
    #[serde(rename = "cold")]
    Cold,
    #[serde(rename = "fire")]
    Fire,
    #[serde(rename = "force")]
    Force,
    #[serde(rename = "lightning")]
    Lightning,
    #[serde(rename = "necrotic")]
    Necrotic,
    #[serde(rename = "piercing")]
    Piercing,
    #[serde(rename = "poison")]
    Poison,
    #[serde(rename = "psychic")]
    Psychic,
    #[serde(rename = "radiant")]
    Radiant,
    #[serde(rename = "slashing")]
    Slashing,
    #[serde(rename = "thunder")]
    Thunder,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "dataOptionalfeatureType")]
pub enum DataOptionalfeatureType {
    #[serde(rename = "ED")]
    Ed,
    #[serde(rename = "EI")]
    Ei,
    #[serde(rename = "MM")]
    Mm,
    #[serde(rename = "MV")]
    Mv,
    #[serde(rename = "MV:B")]
    MvB,
    #[serde(rename = "OTH")]
    Oth,
    #[serde(rename = "FS:F")]
    FsF,
    #[serde(rename = "FS:B")]
    FsB,
    #[serde(rename = "FS:R")]
    FsR,
    #[serde(rename = "FS:P")]
    FsP,
    #[serde(rename = "MV:C2-UA")]
    MvC2Ua,
    #[serde(rename = "AS:V1-UA")]
    AsV1Ua,
    #[serde(rename = "AS:V2-UA")]
    AsV2Ua,
    #[serde(rename = "AS")]
    As,
    #[serde(rename = "PB")]
    Pb,
    #[serde(rename = "AI")]
    Ai,
    #[serde(rename = "SHP:H")]
    ShpH,
    #[serde(rename = "SHP:M")]
    ShpM,
    #[serde(rename = "SHP:W")]
    ShpW,
    #[serde(rename = "SHP:F")]
    ShpF,
    #[serde(rename = "SHP:O")]
    ShpO,
    #[serde(rename = "IWM:W")]
    IwmW,
    #[serde(rename = "IWM:A")]
    IwmA,
    #[serde(rename = "IWM:G")]
    IwmG,
    #[serde(rename = "OR")]
    Or,
    #[serde(rename = "RN")]
    Rn,
    #[serde(rename = "AF")]
    Af,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct ExpertiseItemFrom {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acrobatics: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "alchemist's supplies")]
    pub alchemist_s_supplies: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "animal handling")]
    pub animal_handling: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "anyProficientSkill")]
    pub any_proficient_skill: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "anyProficientTool")]
    pub any_proficient_tool: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arcana: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "artisan's tools")]
    pub artisan_s_tools: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub athletics: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "brewer's supplies")]
    pub brewer_s_supplies: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "calligrapher's supplies")]
    pub calligrapher_s_supplies: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "carpenter's tools")]
    pub carpenter_s_tools: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cartographer's tools")]
    pub cartographer_s_tools: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cobbler's tools")]
    pub cobbler_s_tools: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cook's utensils")]
    pub cook_s_utensils: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deception: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "disguise kit")]
    pub disguise_kit: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "forgery kit")]
    pub forgery_kit: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "gaming set")]
    pub gaming_set: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "glassblower's tools")]
    pub glassblower_s_tools: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "herbalism kit")]
    pub herbalism_kit: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intimidation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub investigation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "jeweler's tools")]
    pub jeweler_s_tools: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "leatherworker's tools")]
    pub leatherworker_s_tools: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mason's tools")]
    pub mason_s_tools: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medicine: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "musical instrument")]
    pub musical_instrument: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nature: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "navigator's tools")]
    pub navigator_s_tools: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "painter's supplies")]
    pub painter_s_supplies: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perception: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persuasion: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "poisoner's kit")]
    pub poisoner_s_kit: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "potter's tools")]
    pub potter_s_tools: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub religion: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sleight of hand")]
    pub sleight_of_hand: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "smith's tools")]
    pub smith_s_tools: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stealth: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub survival: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "thieves' tools")]
    pub thieves_tools: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tinker's tools")]
    pub tinker_s_tools: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "vehicles (land)")]
    pub vehicles_land_: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "vehicles (water)")]
    pub vehicles_water_: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "weaver's tools")]
    pub weaver_s_tools: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "woodcarver's tools")]
    pub woodcarver_s_tools: Option<bool>,
}
pub type Expertise = Vec<ExpertiseItemFrom>;
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(rename = "fluffObject")]
pub struct FluffObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<EntryJson>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<EntryImage>>,
}
pub type GenericFluffArray = Vec<serde_json::Value>;
pub type GenericFluffArrayItemDataImagesVariant0 = Vec<EntryImage>;
pub type GenericFluffArrayItemDataImagesVariant1 = serde_json::Value;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum GenericFluffArrayItemDataImages {
    Variant0(GenericFluffArrayItemDataImagesVariant0),
    Variant1(GenericFluffArrayItemDataImagesVariant1),
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(rename = "genericFluffArrayItemData")]
pub struct GenericFluffArrayItemData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<EntryJson>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<GenericFluffArrayItemDataImages>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "languageNameLower")]
pub enum LanguageNameLower {
    #[serde(rename = "abyssal")]
    Abyssal,
    #[serde(rename = "aquan")]
    Aquan,
    #[serde(rename = "auran")]
    Auran,
    #[serde(rename = "celestial")]
    Celestial,
    #[serde(rename = "common")]
    Common,
    #[serde(rename = "deep speech")]
    DeepSpeech,
    #[serde(rename = "draconic")]
    Draconic,
    #[serde(rename = "druidic")]
    Druidic,
    #[serde(rename = "dwarvish")]
    Dwarvish,
    #[serde(rename = "elvish")]
    Elvish,
    #[serde(rename = "giant")]
    Giant,
    #[serde(rename = "gnomish")]
    Gnomish,
    #[serde(rename = "goblin")]
    Goblin,
    #[serde(rename = "halfling")]
    Halfling,
    #[serde(rename = "ignan")]
    Ignan,
    #[serde(rename = "infernal")]
    Infernal,
    #[serde(rename = "orc")]
    Orc,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "primordial")]
    Primordial,
    #[serde(rename = "sylvan")]
    Sylvan,
    #[serde(rename = "terran")]
    Terran,
    #[serde(rename = "thieves' cant")]
    ThievesCant,
    #[serde(rename = "undercommon")]
    Undercommon,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct LanguageProficienciesItemSourceChoose {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<Vec<LanguageNameLower>>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct LanguageProficienciesItemSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abyssal: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub any: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "anyStandard")]
    pub any_standard: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub celestial: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choose: Option<LanguageProficienciesItemSourceChoose>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "deep speech")]
    pub deep_speech: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draconic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dwarvish: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elvish: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giant: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infernal: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primordial: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sylvan: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub undercommon: Option<bool>,
}
pub type LanguageProficiencies = Vec<LanguageProficienciesItemSource>;
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct MetaBlockDependencies {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monster: Option<Vec<String>>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct MetaBlockOtherSources {
    #[doc = " Keys are other sources to be loaded; values are `otherSources` sources from that source to "]
    #[doc = " search for."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monster: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(rename = "metaBlock")]
pub struct MetaBlock {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<MetaBlockDependencies>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "otherSources")]
    pub other_sources: Option<MetaBlockOtherSources>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct OtherSourcesItemMonster {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    pub source: String,
}
pub type OtherSources = Vec<OtherSourcesItemMonster>;
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct PrerequisiteItemSourceItemAbility {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cha: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub con: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dex: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub int: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub str: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wis: Option<i64>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct PrerequisiteItemSourceLevelVariant0Class {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = " Governs whether or not the class name is visible in the list display/prerequisite line. "]
    #[doc = " *Not* recommended for features which implicitly carry a class restriction, such as Eldritch "]
    #[doc = " Invocations."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct PrerequisiteItemSourceLevelVariant0Subclass {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = " Governs whether or not the class name is visible in the list display/prerequisite line. "]
    #[doc = " *Not* recommended for features which implicitly carry a class restriction, such as Eldritch "]
    #[doc = " Invocations."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct PrerequisiteItemSourceLevelVariant0 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<PrerequisiteItemSourceLevelVariant0Class>,
    pub level: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subclass: Option<PrerequisiteItemSourceLevelVariant0Subclass>,
}
pub type PrerequisiteItemSourceLevelVariant1 = i64;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PrerequisiteItemSourceLevel {
    Variant0(PrerequisiteItemSourceLevelVariant0),
    Variant1(PrerequisiteItemSourceLevelVariant1),
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct PrerequisiteItemSourceOtherSummary {
    pub entry: String,
    #[doc = " Used in short/list displays"]
    #[serde(rename = "entrySummary")]
    pub entry_summary: String,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct PrerequisiteItemSourceItemProficiency {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub armor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weapon: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct PrerequisiteItemSourceItemRace {
    #[doc = " Optional long-form name to be used in the rendered entity."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "displayEntry")]
    pub display_entry: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subrace: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct PrerequisiteItemSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ability: Option<Vec<PrerequisiteItemSourceItemAbility>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature: Option<Vec<EntryJson>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Vec<EntryJson>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<PrerequisiteItemSourceLevel>,
    #[doc = " A free text prerequisite"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<String>,
    #[doc = " A free text prerequisite, with a shortened form for list display."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "otherSummary")]
    pub other_summary: Option<PrerequisiteItemSourceOtherSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pact: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patron: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proficiency: Option<Vec<PrerequisiteItemSourceItemProficiency>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psionics: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub race: Option<Vec<PrerequisiteItemSourceItemRace>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spell: Option<Vec<EntryJson>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spellcasting: Option<bool>,
    #[doc = " Renders with the updated text found in UA2020: Feats"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "spellcasting2020")]
    pub spellcasting_2020: Option<bool>,
}
pub type Prerequisite = Vec<PrerequisiteItemSource>;
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct ReqAttuneTagsItemSpellcasting2020 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment: Option<Vec<Alignment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cha: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub con: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "creatureType")]
    pub creature_type: Option<CreatureType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dex: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub int: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "languageProficiency")]
    pub language_proficiency: Option<LanguageNameLower>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psionics: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub race: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "skillProficiency")]
    pub skill_proficiency: Option<SkillNameLower>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spellcasting: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub str: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wis: Option<i64>,
}
pub type ReqAttuneTags = Vec<ReqAttuneTagsItemSpellcasting2020>;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SavingThrowProficienciesItemWisChoose {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    pub from: Vec<serde_json::Value>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct SavingThrowProficienciesItemWis {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cha: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choose: Option<SavingThrowProficienciesItemWisChoose>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub con: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dex: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub int: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub str: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wis: Option<bool>,
}
pub type SavingThrowProficiencies = Vec<SavingThrowProficienciesItemWis>;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "size")]
pub enum Size {
    F,
    D,
    T,
    S,
    M,
    L,
    H,
    G,
    C,
    V,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "skillNameLower")]
pub enum SkillNameLower {
    #[serde(rename = "athletics")]
    Athletics,
    #[serde(rename = "acrobatics")]
    Acrobatics,
    #[serde(rename = "sleight of hand")]
    SleightOfHand,
    #[serde(rename = "stealth")]
    Stealth,
    #[serde(rename = "arcana")]
    Arcana,
    #[serde(rename = "history")]
    History,
    #[serde(rename = "investigation")]
    Investigation,
    #[serde(rename = "nature")]
    Nature,
    #[serde(rename = "religion")]
    Religion,
    #[serde(rename = "animal handling")]
    AnimalHandling,
    #[serde(rename = "insight")]
    Insight,
    #[serde(rename = "medicine")]
    Medicine,
    #[serde(rename = "perception")]
    Perception,
    #[serde(rename = "survival")]
    Survival,
    #[serde(rename = "deception")]
    Deception,
    #[serde(rename = "intimidation")]
    Intimidation,
    #[serde(rename = "performance")]
    Performance,
    #[serde(rename = "persuasion")]
    Persuasion,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct SkillProficienciesItemWisChooseItemFromVariant1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool: Option<Vec<String>>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct SkillProficienciesItemWisChooseItemFromVariant2 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SkillProficienciesItemWisChooseItemFrom {
    Variant0(SkillNameLower),
    Variant1(SkillProficienciesItemWisChooseItemFromVariant1),
    Variant2(SkillProficienciesItemWisChooseItemFromVariant2),
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct SkillProficienciesItemWisChoose {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<Vec<SkillProficienciesItemWisChooseItemFrom>>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct SkillProficienciesItemWis {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acrobatics: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "animal handling")]
    pub animal_handling: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arcana: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub athletics: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choose: Option<SkillProficienciesItemWisChoose>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deception: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intimidation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub investigation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medicine: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nature: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perception: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persuasion: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub religion: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sleight of hand")]
    pub sleight_of_hand: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stealth: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub survival: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool: Option<bool>,
}
pub type SkillProficiencies = Vec<SkillProficienciesItemWis>;
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct SpeedToolVariant0Alternate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub burrow: Option<Vec<SpeedVal>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub climb: Option<Vec<SpeedVal>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fly: Option<Vec<SpeedVal>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swim: Option<Vec<SpeedVal>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub walk: Option<Vec<SpeedVal>>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SpeedToolVariant0Choose {
    pub amount: i64,
    pub from: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct SpeedToolVariant0 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternate: Option<SpeedToolVariant0Alternate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub burrow: Option<SpeedVal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "canHover")]
    pub can_hover: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choose: Option<SpeedToolVariant0Choose>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub climb: Option<SpeedVal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fly: Option<SpeedVal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swim: Option<SpeedVal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub walk: Option<SpeedVal>,
}
pub type SpeedToolVariant1 = i64;
pub type SpeedToolVariant2 = serde_json::Value;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SpeedTool {
    Variant0(SpeedToolVariant0),
    Variant1(SpeedToolVariant1),
    Variant2(SpeedToolVariant2),
}
pub type Speed = SpeedTool;
pub type SrdWalkVariant0 = bool;
pub type SrdWalkVariant1 = String;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SrdWalk {
    Variant0(SrdWalkVariant0),
    Variant1(SrdWalkVariant1),
}
pub type Srd = SrdWalk;
#[doc = " Optional well-structured data version of the \"default\" property, for use in applications that "]
#[doc = " require it."]
pub type StartingEquipment = Vec<::std::collections::BTreeMap<String, serde_json::Value>>;
pub type TagsConditions = Vec<String>;
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct ToolProficienciesItemWalkChoose {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<Vec<serde_json::Value>>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct ToolProficienciesItemWalk {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "alchemist's supplies")]
    pub alchemist_s_supplies: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub any: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "artisan's tools")]
    pub artisan_s_tools: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "brewer's supplies")]
    pub brewer_s_supplies: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "calligrapher's supplies")]
    pub calligrapher_s_supplies: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "carpenter's tools")]
    pub carpenter_s_tools: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cartographer's tools")]
    pub cartographer_s_tools: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choose: Option<ToolProficienciesItemWalkChoose>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cobbler's tools")]
    pub cobbler_s_tools: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cook's utensils")]
    pub cook_s_utensils: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "disguise kit")]
    pub disguise_kit: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "forgery kit")]
    pub forgery_kit: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "gaming set")]
    pub gaming_set: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "glassblower's tools")]
    pub glassblower_s_tools: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "herbalism kit")]
    pub herbalism_kit: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "jeweler's tools")]
    pub jeweler_s_tools: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "leatherworker's tools")]
    pub leatherworker_s_tools: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mason's tools")]
    pub mason_s_tools: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "musical instrument")]
    pub musical_instrument: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "navigator's tools")]
    pub navigator_s_tools: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "painter's supplies")]
    pub painter_s_supplies: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "poisoner's kit")]
    pub poisoner_s_kit: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "potter's tools")]
    pub potter_s_tools: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "smith's tools")]
    pub smith_s_tools: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "thieves' tools")]
    pub thieves_tools: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tinker's tools")]
    pub tinker_s_tools: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "vehicles (land)")]
    pub vehicles_land_: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "vehicles (water)")]
    pub vehicles_water_: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "weaver's tools")]
    pub weaver_s_tools: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "woodcarver's tools")]
    pub woodcarver_s_tools: Option<bool>,
}
pub type ToolProficiencies = Vec<ToolProficienciesItemWalk>;
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct WeaponProficienciesItemWoodcarverSToolsChoose {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = " A filter string, e.g. \"type=martial weapon|miscellaneous=mundane\""]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct WeaponProficienciesItemWoodcarverSTools {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "battleaxe|phb")]
    pub battleaxe_phb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "blowgun|phb")]
    pub blowgun_phb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choose: Option<WeaponProficienciesItemWoodcarverSToolsChoose>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "club|phb")]
    pub club_phb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dagger|phb")]
    pub dagger_phb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dart|phb")]
    pub dart_phb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "double-bladed scimitar|erlw")]
    pub double_bladed_scimitar_erlw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firearms: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "flail|phb")]
    pub flail_phb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "glaive|phb")]
    pub glaive_phb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "greataxe|phb")]
    pub greataxe_phb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "greatclub|phb")]
    pub greatclub_phb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "greatsword|phb")]
    pub greatsword_phb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "halberd|phb")]
    pub halberd_phb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hand crossbow|phb")]
    pub hand_crossbow_phb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "handaxe|phb")]
    pub handaxe_phb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "heavy crossbow|phb")]
    pub heavy_crossbow_phb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "javelin|phb")]
    pub javelin_phb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "lance|phb")]
    pub lance_phb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "light crossbow|phb")]
    pub light_crossbow_phb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "light hammer|phb")]
    pub light_hammer_phb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "longbow|phb")]
    pub longbow_phb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "longsword|phb")]
    pub longsword_phb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mace|phb")]
    pub mace_phb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub martial: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "maul|phb")]
    pub maul_phb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "morningstar|phb")]
    pub morningstar_phb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "net|phb")]
    pub net_phb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pike|phb")]
    pub pike_phb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "quarterstaff|phb")]
    pub quarterstaff_phb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "rapier|phb")]
    pub rapier_phb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "scimitar|phb")]
    pub scimitar_phb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "shortbow|phb")]
    pub shortbow_phb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "shortsword|phb")]
    pub shortsword_phb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sickle|phb")]
    pub sickle_phb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sling|phb")]
    pub sling_phb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "spear|phb")]
    pub spear_phb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "staff|phb")]
    pub staff_phb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "trident|phb")]
    pub trident_phb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "war pick|phb")]
    pub war_pick_phb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "warhammer|phb")]
    pub warhammer_phb: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "whip|phb")]
    pub whip_phb: Option<bool>,
}
pub type WeaponProficiencies = Vec<WeaponProficienciesItemWoodcarverSTools>;
