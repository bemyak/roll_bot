use super::*;
use serde::{Deserialize, Serialize};
pub type AcItem = serde_json::Value;
pub type Align = serde_json::Value;
pub type Creature = serde_json::Value;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CreatureDataItemItemItemItemItemAltArt {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    pub source: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CreatureDataItemItemItemItemItemAltArtItemBonus {
    pub entries: Vec<EntryJson>,
    pub name: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CreatureDataItemItemItemItemItemAltArtItemBonusItemItemItemItemExternalSources {
    pub entry: EntryJson,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct CreatureDataItemItemItemItemItemAltArtItemBonusItemItemItemItemExternalSourcesItemItemLegendaryGroup
{
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CreatureDataItemItemItemItemItemAltArtItemBonusItemItemItemItemExternalSourcesItemItemLegendaryGroupItemItemItemMythic
{
    pub entries: Vec<EntryJson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CreatureDataItemItemItemItemItemAltArtItemBonusItemItemItemItemExternalSourcesItemItemLegendaryGroupItemItemItemMythicItemItemItemReaction
{
    pub entries: Vec<EntryJson>,
    pub name: String,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct CreatureDataItemItemItemItemItemAltArtItemBonusItemItemItemItemExternalSourcesItemItemLegendaryGroupItemItemItemMythicItemItemItemReactionItemSave
{
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
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct CreatureDataItemItemItemItemItemAltArtItemBonusItemItemItemItemExternalSourcesItemItemLegendaryGroupItemItemItemMythicItemItemItemReactionItemSaveItemSkillItemOtherOneOf
{
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
pub struct CreatureDataItemItemItemItemItemAltArtItemBonusItemItemItemItemExternalSourcesItemItemLegendaryGroupItemItemItemMythicItemItemItemReactionItemSaveItemSkillItemOther { # [serde (skip_serializing_if = "Option::is_none")] # [serde (rename = "oneOf")] pub one_of : Option < CreatureDataItemItemItemItemItemAltArtItemBonusItemItemItemItemExternalSourcesItemItemLegendaryGroupItemItemItemMythicItemItemItemReactionItemSaveItemSkillItemOtherOneOf > }
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct CreatureDataItemItemItemItemItemAltArtItemBonusItemItemItemItemExternalSourcesItemItemLegendaryGroupItemItemItemMythicItemItemItemReactionItemSaveItemSkill { # [serde (skip_serializing_if = "Option::is_none")] pub acrobatics : Option < String > , # [serde (skip_serializing_if = "Option::is_none")] # [serde (rename = "animal handling")] pub animal_handling : Option < String > , # [serde (skip_serializing_if = "Option::is_none")] pub arcana : Option < String > , # [serde (skip_serializing_if = "Option::is_none")] pub athletics : Option < String > , # [serde (skip_serializing_if = "Option::is_none")] pub deception : Option < String > , # [serde (skip_serializing_if = "Option::is_none")] pub history : Option < String > , # [serde (skip_serializing_if = "Option::is_none")] pub insight : Option < String > , # [serde (skip_serializing_if = "Option::is_none")] pub intimidation : Option < String > , # [serde (skip_serializing_if = "Option::is_none")] pub investigation : Option < String > , # [serde (skip_serializing_if = "Option::is_none")] pub medicine : Option < String > , # [serde (skip_serializing_if = "Option::is_none")] pub nature : Option < String > , # [serde (skip_serializing_if = "Option::is_none")] pub other : Option < Vec < CreatureDataItemItemItemItemItemAltArtItemBonusItemItemItemItemExternalSourcesItemItemLegendaryGroupItemItemItemMythicItemItemItemReactionItemSaveItemSkillItemOther >> , # [serde (skip_serializing_if = "Option::is_none")] pub perception : Option < String > , # [serde (skip_serializing_if = "Option::is_none")] pub performance : Option < String > , # [serde (skip_serializing_if = "Option::is_none")] pub persuasion : Option < String > , # [serde (skip_serializing_if = "Option::is_none")] pub religion : Option < String > , # [serde (skip_serializing_if = "Option::is_none")] # [serde (rename = "sleight of hand")] pub sleight_of_hand : Option < String > , # [doc = " For use in homebrew."] # [serde (skip_serializing_if = "Option::is_none")] pub special : Option < EntryJson > , # [serde (skip_serializing_if = "Option::is_none")] pub stealth : Option < String > , # [serde (skip_serializing_if = "Option::is_none")] pub survival : Option < String > }
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CreatureDataItemItemItemItemItemAltArtItemBonusItemItemItemItemExternalSourcesItemItemLegendaryGroupItemItemItemMythicItemItemItemReactionItemSaveItemSkillItemOtherOneOfItemItemItemVariantItemToken
{
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    pub source: String,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct CreatureDataItemItemItemItemItemAltArtItemBonusItemItemItemItemExternalSourcesItemItemLegendaryGroupItemItemItemMythicItemItemItemReactionItemSaveItemSkillItemOtherOneOfItemItemItemVariantItemTokenVariantSource
{
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct CreatureDataItemItemItemItemItemAltArtItemBonusItemItemItemItemExternalSourcesItemItemLegendaryGroupItemItemItemMythicItemItemItemReactionItemSaveItemSkillItemOtherOneOfItemItemItemVariant { # [serde (skip_serializing_if = "Option::is_none")] pub entries : Option < Vec < EntryJson >> , # [serde (skip_serializing_if = "Option::is_none")] pub name : Option < String > , # [serde (skip_serializing_if = "Option::is_none")] pub token : Option < CreatureDataItemItemItemItemItemAltArtItemBonusItemItemItemItemExternalSourcesItemItemLegendaryGroupItemItemItemMythicItemItemItemReactionItemSaveItemSkillItemOtherOneOfItemItemItemVariantItemToken > , # [serde (skip_serializing_if = "Option::is_none")] # [serde (rename = "type")] pub type_ : Option < String > , # [serde (skip_serializing_if = "Option::is_none")] # [serde (rename = "variantSource")] pub variant_source : Option < CreatureDataItemItemItemItemItemAltArtItemBonusItemItemItemItemExternalSourcesItemItemLegendaryGroupItemItemItemMythicItemItemItemReactionItemSaveItemSkillItemOtherOneOfItemItemItemVariantItemTokenVariantSource > }
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(rename = "creatureData")]
pub struct CreatureData { # [doc = " An internal flag indicating this creature is a copy of another, and is a "] # [doc = " temporary/placeholder entry which will be factored out using the \"_copy\" format at a later "] # [doc = " date."] # [serde (skip_serializing_if = "Option::is_none")] # [serde (rename = "_isCopy")] pub is_copy : Option < bool > , # [serde (skip_serializing_if = "Option::is_none")] pub ac : Option < Vec < AcItem >> , # [serde (skip_serializing_if = "Option::is_none")] pub action : Option < serde_json :: Value > , # [serde (skip_serializing_if = "Option::is_none")] # [serde (rename = "actionNote")] pub action_note : Option < String > , # [serde (skip_serializing_if = "Option::is_none")] # [serde (rename = "actionTags")] pub action_tags : Option < Vec < String >> , # [serde (skip_serializing_if = "Option::is_none")] # [serde (rename = "additionalSources")] pub additional_sources : Option < AdditionalSources > , # [serde (skip_serializing_if = "Option::is_none")] pub alias : Option < Vec < String >> , # [serde (skip_serializing_if = "Option::is_none")] pub alignment : Option < Vec < Align >> , # [serde (skip_serializing_if = "Option::is_none")] # [serde (rename = "altArt")] pub alt_art : Option < Vec < CreatureDataItemItemItemItemItemAltArt >> , # [serde (skip_serializing_if = "Option::is_none")] pub bonus : Option < Vec < CreatureDataItemItemItemItemItemAltArtItemBonus >> , # [serde (skip_serializing_if = "Option::is_none")] pub cha : Option < i64 > , # [serde (skip_serializing_if = "Option::is_none")] pub con : Option < i64 > , # [serde (skip_serializing_if = "Option::is_none")] # [serde (rename = "conditionImmune")] pub condition_immune : Option < ConditionImmunityArray > , # [serde (skip_serializing_if = "Option::is_none")] # [serde (rename = "conditionInflict")] pub condition_inflict : Option < TagsConditions > , # [serde (skip_serializing_if = "Option::is_none")] # [serde (rename = "conditionInflictLegendary")] pub condition_inflict_legendary : Option < TagsConditions > , # [serde (skip_serializing_if = "Option::is_none")] # [serde (rename = "conditionInflictSpell")] pub condition_inflict_spell : Option < TagsConditions > , # [serde (skip_serializing_if = "Option::is_none")] pub cr : Option < serde_json :: Value > , # [serde (skip_serializing_if = "Option::is_none")] # [serde (rename = "damageTags")] pub damage_tags : Option < Vec < String >> , # [serde (skip_serializing_if = "Option::is_none")] pub dex : Option < i64 > , # [serde (skip_serializing_if = "Option::is_none")] # [serde (rename = "dragonCastingColor")] pub dragon_casting_color : Option < String > , # [serde (skip_serializing_if = "Option::is_none")] pub environment : Option < Vec < String >> , # [doc = " For homebrew use only."] # [serde (skip_serializing_if = "Option::is_none")] # [serde (rename = "externalSources")] pub external_sources : Option < Vec < CreatureDataItemItemItemItemItemAltArtItemBonusItemItemItemItemExternalSources >> , # [serde (skip_serializing_if = "Option::is_none")] pub familiar : Option < bool > , # [doc = " This is intended to be used for Homebrew only - site data should include a fluff file per "] # [doc = " source"] # [serde (skip_serializing_if = "Option::is_none")] pub fluff : Option < serde_json :: Value > , # [doc = " Intended for homebrew use only."] # [serde (skip_serializing_if = "Option::is_none")] pub footer : Option < Vec < EntryJson >> , # [doc = " A group name, indexed by search. E.g. searching \"Lycanthrope\" would otherwise fail to find "] # [doc = " anything"] # [serde (default)] pub group : Option < String > , # [serde (skip_serializing_if = "Option::is_none")] # [serde (rename = "hasFluff")] pub has_fluff : Option < bool > , # [serde (skip_serializing_if = "Option::is_none")] # [serde (rename = "hasFluffImages")] pub has_fluff_images : Option < bool > , # [serde (skip_serializing_if = "Option::is_none")] # [serde (rename = "hasToken")] pub has_token : Option < bool > , # [serde (skip_serializing_if = "Option::is_none")] pub hp : Option < serde_json :: Value > , # [serde (skip_serializing_if = "Option::is_none")] pub immune : Option < DamageImmunityArray > , # [serde (skip_serializing_if = "Option::is_none")] pub int : Option < i64 > , # [serde (skip_serializing_if = "Option::is_none")] # [serde (rename = "isNamedCreature")] pub is_named_creature : Option < bool > , # [doc = " Used to flag adventure NPCs"] # [serde (skip_serializing_if = "Option::is_none")] # [serde (rename = "isNpc")] pub is_npc : Option < bool > , # [serde (skip_serializing_if = "Option::is_none")] # [serde (rename = "languageTags")] pub language_tags : Option < Vec < String >> , # [serde (skip_serializing_if = "Option::is_none")] pub languages : Option < serde_json :: Value > , # [serde (skip_serializing_if = "Option::is_none")] pub legendary : Option < serde_json :: Value > , # [serde (skip_serializing_if = "Option::is_none")] # [serde (rename = "legendaryActions")] pub legendary_actions : Option < f64 > , # [serde (skip_serializing_if = "Option::is_none")] # [serde (rename = "legendaryGroup")] pub legendary_group : Option < CreatureDataItemItemItemItemItemAltArtItemBonusItemItemItemItemExternalSourcesItemItemLegendaryGroup > , # [serde (skip_serializing_if = "Option::is_none")] # [serde (rename = "legendaryHeader")] pub legendary_header : Option < Vec < EntryJson >> , # [doc = " Used in sidekicks, which can have levels (and generally do not have alignment)"] # [serde (skip_serializing_if = "Option::is_none")] pub level : Option < i64 > , # [serde (skip_serializing_if = "Option::is_none")] # [serde (rename = "miscTags")] pub misc_tags : Option < Vec < String >> , # [serde (skip_serializing_if = "Option::is_none")] pub mythic : Option < Vec < CreatureDataItemItemItemItemItemAltArtItemBonusItemItemItemItemExternalSourcesItemItemLegendaryGroupItemItemItemMythic >> , # [serde (skip_serializing_if = "Option::is_none")] # [serde (rename = "mythicHeader")] pub mythic_header : Option < Vec < EntryJson >> , # [serde (skip_serializing_if = "Option::is_none")] pub name : Option < String > , # [serde (skip_serializing_if = "Option::is_none")] # [serde (rename = "otherSources")] pub other_sources : Option < OtherSources > , # [serde (skip_serializing_if = "Option::is_none")] pub page : Option < i64 > , # [serde (skip_serializing_if = "Option::is_none")] pub passive : Option < serde_json :: Value > , # [serde (skip_serializing_if = "Option::is_none")] # [serde (rename = "pbNote")] pub pb_note : Option < String > , # [serde (skip_serializing_if = "Option::is_none")] pub reaction : Option < Vec < CreatureDataItemItemItemItemItemAltArtItemBonusItemItemItemItemExternalSourcesItemItemLegendaryGroupItemItemItemMythicItemItemItemReaction >> , # [serde (skip_serializing_if = "Option::is_none")] pub resist : Option < DamageResistArray > , # [serde (skip_serializing_if = "Option::is_none")] pub save : Option < CreatureDataItemItemItemItemItemAltArtItemBonusItemItemItemItemExternalSourcesItemItemLegendaryGroupItemItemItemMythicItemItemItemReactionItemSave > , # [serde (skip_serializing_if = "Option::is_none")] # [serde (rename = "senseTags")] pub sense_tags : Option < Vec < String >> , # [serde (skip_serializing_if = "Option::is_none")] pub senses : Option < serde_json :: Value > , # [serde (skip_serializing_if = "Option::is_none")] # [serde (rename = "shortName")] pub short_name : Option < serde_json :: Value > , # [serde (skip_serializing_if = "Option::is_none")] pub size : Option < Size > , # [serde (skip_serializing_if = "Option::is_none")] # [serde (rename = "sizeNote")] pub size_note : Option < String > , # [serde (skip_serializing_if = "Option::is_none")] pub skill : Option < CreatureDataItemItemItemItemItemAltArtItemBonusItemItemItemItemExternalSourcesItemItemLegendaryGroupItemItemItemMythicItemItemItemReactionItemSaveItemSkill > , # [serde (skip_serializing_if = "Option::is_none")] # [serde (rename = "soundClip")] pub sound_clip : Option < MediaHref > , # [serde (skip_serializing_if = "Option::is_none")] pub source : Option < String > , # [serde (skip_serializing_if = "Option::is_none")] pub speed : Option < Speed > , # [serde (skip_serializing_if = "Option::is_none")] pub spellcasting : Option < serde_json :: Value > , # [serde (skip_serializing_if = "Option::is_none")] # [serde (rename = "spellcastingTags")] pub spellcasting_tags : Option < Vec < String >> , # [serde (skip_serializing_if = "Option::is_none")] pub srd : Option < Srd > , # [serde (skip_serializing_if = "Option::is_none")] pub str : Option < i64 > , # [doc = " The spell used to summon this creature; specifically for TCE-esque summon spells."] # [serde (skip_serializing_if = "Option::is_none")] # [serde (rename = "summonedBySpell")] pub summoned_by_spell : Option < String > , # [serde (skip_serializing_if = "Option::is_none")] # [serde (rename = "tokenUrl")] pub token_url : Option < String > , # [serde (skip_serializing_if = "Option::is_none")] # [serde (rename = "trait")] pub trait_ : Option < serde_json :: Value > , # [serde (skip_serializing_if = "Option::is_none")] # [serde (rename = "traitTags")] pub trait_tags : Option < Vec < String >> , # [serde (skip_serializing_if = "Option::is_none")] # [serde (rename = "type")] pub type_ : Option < serde_json :: Value > , # [serde (skip_serializing_if = "Option::is_none")] pub variant : Option < Vec < CreatureDataItemItemItemItemItemAltArtItemBonusItemItemItemItemExternalSourcesItemItemLegendaryGroupItemItemItemMythicItemItemItemReactionItemSaveItemSkillItemOtherOneOfItemItemItemVariant >> , # [serde (skip_serializing_if = "Option::is_none")] pub vulnerable : Option < DamageVulnerabilityArray > , # [serde (skip_serializing_if = "Option::is_none")] pub wis : Option < i64 > }
