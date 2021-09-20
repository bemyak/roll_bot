use super::*;
use serde::{Deserialize, Serialize};
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct AcItemVariant0 {
    pub ac: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub braces: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<Vec<String>>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct AcItemVariant1 {
    pub special: String,
}
pub type AcItemVariant2 = i64;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AcItem {
    Variant0(AcItemVariant0),
    Variant1(AcItemVariant1),
    Variant2(AcItemVariant2),
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum AlignSpecialVariant0 {
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
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct AlignSpecialVariant1 {
    pub alignment: Vec<Alignment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chance: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct AlignSpecialVariant2 {
    pub special: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AlignSpecial {
    Variant0(AlignSpecialVariant0),
    Variant1(AlignSpecialVariant1),
    Variant2(AlignSpecialVariant2),
}
pub type Align = AlignSpecial;
pub type Creature = serde_json::Value;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CreatureDataActionVariant0ItemAction {
    pub entries: Vec<EntryJson>,
    pub name: String,
}
pub type CreatureDataActionVariant0 = Vec<CreatureDataActionVariant0ItemAction>;
pub type CreatureDataActionVariant1 = serde_json::Value;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CreatureDataAction {
    Variant0(CreatureDataActionVariant0),
    Variant1(CreatureDataActionVariant1),
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CreatureDataItemAltArt {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    pub source: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CreatureDataItemBonus {
    pub entries: Vec<EntryJson>,
    pub name: String,
}
pub type CreatureDataCrVariant0 = String;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CreatureDataCrVariant1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coven: Option<String>,
    pub cr: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lair: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xp: Option<i64>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CreatureDataCr {
    Variant0(CreatureDataCrVariant0),
    Variant1(CreatureDataCrVariant1),
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CreatureDataItemExternalSources {
    pub entry: EntryJson,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct CreatureDataFluffVariant1AppendMonsterFluff {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct CreatureDataFluffVariant1MonsterFluff {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct CreatureDataFluffVariant1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_appendMonsterFluff")]
    pub append_monster_fluff: Option<CreatureDataFluffVariant1AppendMonsterFluff>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_monsterFluff")]
    pub monster_fluff: Option<CreatureDataFluffVariant1MonsterFluff>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CreatureDataFluff {
    Variant0(FluffObject),
    Variant1(CreatureDataFluffVariant1),
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct CreatureDataHpVariant0 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formula: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct CreatureDataHpVariant1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CreatureDataHp {
    Variant0(CreatureDataHpVariant0),
    Variant1(CreatureDataHpVariant1),
}
pub type CreatureDataLanguagesVariant0 = Vec<String>;
pub type CreatureDataLanguagesVariant1 = serde_json::Value;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CreatureDataLanguages {
    Variant0(CreatureDataLanguagesVariant0),
    Variant1(CreatureDataLanguagesVariant1),
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CreatureDataLegendaryVariant0ItemLegendary {
    pub entries: Vec<EntryJson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
pub type CreatureDataLegendaryVariant0 = Vec<CreatureDataLegendaryVariant0ItemLegendary>;
pub type CreatureDataLegendaryVariant1 = serde_json::Value;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CreatureDataLegendary {
    Variant0(CreatureDataLegendaryVariant0),
    Variant1(CreatureDataLegendaryVariant1),
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct CreatureDataLegendaryGroup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CreatureDataItemMythic {
    pub entries: Vec<EntryJson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
pub type CreatureDataPassiveVariant0 = i64;
pub type CreatureDataPassiveVariant1 = String;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CreatureDataPassive {
    Variant0(CreatureDataPassiveVariant0),
    Variant1(CreatureDataPassiveVariant1),
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CreatureDataItemReaction {
    pub entries: Vec<EntryJson>,
    pub name: String,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct CreatureDataSave {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cha: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub con: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub int: Option<String>,
    #[doc = " For use in homebrew."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special: Option<EntryJson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub str: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wis: Option<String>,
}
pub type CreatureDataSensesVariant0 = Vec<String>;
pub type CreatureDataSensesVariant1 = serde_json::Value;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CreatureDataSenses {
    Variant0(CreatureDataSensesVariant0),
    Variant1(CreatureDataSensesVariant1),
}
pub type CreatureDataShortNameVariant0 = String;
pub type CreatureDataShortNameVariant1 = bool;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CreatureDataShortName {
    Variant0(CreatureDataShortNameVariant0),
    Variant1(CreatureDataShortNameVariant1),
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct CreatureDataSkillItemOtherOneOf {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acrobatics: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "animal handling")]
    pub animal_handling: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arcana: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub athletics: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deception: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intimidation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub investigation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medicine: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perception: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persuasion: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub religion: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sleight of hand")]
    pub sleight_of_hand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stealth: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub survival: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct CreatureDataSkillItemOther {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "oneOf")]
    pub one_of: Option<CreatureDataSkillItemOtherOneOf>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct CreatureDataSkill {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acrobatics: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "animal handling")]
    pub animal_handling: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arcana: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub athletics: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deception: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intimidation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub investigation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medicine: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<Vec<CreatureDataSkillItemOther>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perception: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persuasion: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub religion: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sleight of hand")]
    pub sleight_of_hand: Option<String>,
    #[doc = " For use in homebrew."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special: Option<EntryJson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stealth: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub survival: Option<String>,
}
pub type CreatureDataSpellcastingVariant0 = Vec<EntrySpellcasting>;
pub type CreatureDataSpellcastingVariant1 = serde_json::Value;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CreatureDataSpellcasting {
    Variant0(CreatureDataSpellcastingVariant0),
    Variant1(CreatureDataSpellcastingVariant1),
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CreatureDataTraitVariant0ItemTrait {
    pub entries: Vec<EntryJson>,
    pub name: String,
    #[doc = " Forces a sort order. Traits with sort orders will always be arranged before those without."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
pub type CreatureDataTraitVariant0 = Vec<CreatureDataTraitVariant0ItemTrait>;
pub type CreatureDataTraitVariant1 = serde_json::Value;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CreatureDataTrait {
    Variant0(CreatureDataTraitVariant0),
    Variant1(CreatureDataTraitVariant1),
}
pub type CreatureDataTypeVariant0ItemTagsVariant0 = String;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CreatureDataTypeVariant0ItemTagsVariant1 {
    pub prefix: String,
    pub tag: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CreatureDataTypeVariant0ItemTags {
    Variant0(CreatureDataTypeVariant0ItemTagsVariant0),
    Variant1(CreatureDataTypeVariant0ItemTagsVariant1),
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CreatureDataTypeVariant0 {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "swarmSize")]
    pub swarm_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<CreatureDataTypeVariant0ItemTags>>,
    #[serde(rename = "type")]
    pub type_: CreatureType,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CreatureDataType {
    Variant0(CreatureDataTypeVariant0),
    Variant1(CreatureType),
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CreatureDataItemVariantToken {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    pub source: String,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct CreatureDataItemVariantVariantSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct CreatureDataItemVariant {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<EntryJson>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<CreatureDataItemVariantToken>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "variantSource")]
    pub variant_source: Option<CreatureDataItemVariantVariantSource>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(rename = "creatureData")]
pub struct CreatureData {
    #[doc = " An internal flag indicating this creature is a copy of another, and is a "]
    #[doc = " temporary/placeholder entry which will be factored out using the \"_copy\" format at a later "]
    #[doc = " date."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_isCopy")]
    pub is_copy: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ac: Option<Vec<AcItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<CreatureDataAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "actionNote")]
    pub action_note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "actionTags")]
    pub action_tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "additionalSources")]
    pub additional_sources: Option<AdditionalSources>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment: Option<Vec<Align>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "altArt")]
    pub alt_art: Option<Vec<CreatureDataItemAltArt>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bonus: Option<Vec<CreatureDataItemBonus>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cha: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub con: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "conditionImmune")]
    pub condition_immune: Option<ConditionImmunityArray>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "conditionInflict")]
    pub condition_inflict: Option<TagsConditions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "conditionInflictLegendary")]
    pub condition_inflict_legendary: Option<TagsConditions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "conditionInflictSpell")]
    pub condition_inflict_spell: Option<TagsConditions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cr: Option<CreatureDataCr>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "damageTags")]
    pub damage_tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dex: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dragonCastingColor")]
    pub dragon_casting_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Vec<String>>,
    #[doc = " For homebrew use only."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "externalSources")]
    pub external_sources: Option<Vec<CreatureDataItemExternalSources>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub familiar: Option<bool>,
    #[doc = " This is intended to be used for Homebrew only - site data should include a fluff file per "]
    #[doc = " source"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fluff: Option<CreatureDataFluff>,
    #[doc = " Intended for homebrew use only."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<Vec<EntryJson>>,
    #[doc = " A group name, indexed by search. E.g. searching \"Lycanthrope\" would otherwise fail to find "]
    #[doc = " anything"]
    #[serde(default)]
    pub group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hasFluff")]
    pub has_fluff: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hasFluffImages")]
    pub has_fluff_images: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hasToken")]
    pub has_token: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hp: Option<CreatureDataHp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub immune: Option<DamageImmunityArray>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub int: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "isNamedCreature")]
    pub is_named_creature: Option<bool>,
    #[doc = " Used to flag adventure NPCs"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "isNpc")]
    pub is_npc: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "languageTags")]
    pub language_tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages: Option<CreatureDataLanguages>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legendary: Option<CreatureDataLegendary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "legendaryActions")]
    pub legendary_actions: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "legendaryGroup")]
    pub legendary_group: Option<CreatureDataLegendaryGroup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "legendaryHeader")]
    pub legendary_header: Option<Vec<EntryJson>>,
    #[doc = " Used in sidekicks, which can have levels (and generally do not have alignment)"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "miscTags")]
    pub misc_tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mythic: Option<Vec<CreatureDataItemMythic>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mythicHeader")]
    pub mythic_header: Option<Vec<EntryJson>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "otherSources")]
    pub other_sources: Option<OtherSources>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passive: Option<CreatureDataPassive>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pbNote")]
    pub pb_note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reaction: Option<Vec<CreatureDataItemReaction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resist: Option<DamageResistArray>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save: Option<CreatureDataSave>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "senseTags")]
    pub sense_tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub senses: Option<CreatureDataSenses>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "shortName")]
    pub short_name: Option<CreatureDataShortName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<Size>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sizeNote")]
    pub size_note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill: Option<CreatureDataSkill>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "soundClip")]
    pub sound_clip: Option<MediaHref>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<Speed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spellcasting: Option<CreatureDataSpellcasting>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "spellcastingTags")]
    pub spellcasting_tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srd: Option<Srd>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub str: Option<i64>,
    #[doc = " The spell used to summon this creature; specifically for TCE-esque summon spells."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "summonedBySpell")]
    pub summoned_by_spell: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tokenUrl")]
    pub token_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "trait")]
    pub trait_: Option<CreatureDataTrait>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "traitTags")]
    pub trait_tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<CreatureDataType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<Vec<CreatureDataItemVariant>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulnerable: Option<DamageVulnerabilityArray>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wis: Option<i64>,
}
