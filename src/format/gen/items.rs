use super::*;
use serde::{Deserialize, Serialize};
pub type Item = serde_json::Value;
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct ItemDataAbility {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cha: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choose: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub con: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dex: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub int: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "static")]
    pub static_: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub str: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wis: Option<i64>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct ItemDataContainerCapacityItemItem {}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct ItemDataContainerCapacity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Vec<ItemDataContainerCapacityItemItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<Vec<i64>>,
    #[doc = " If the container renders its contents weightless."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weightless: Option<bool>,
}
pub type ItemDataFocusVariant0 = bool;
pub type ItemDataFocusVariant1 = Vec<String>;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ItemDataFocus {
    Variant0(ItemDataFocusVariant0),
    Variant1(ItemDataFocusVariant1),
}
pub type ItemDataItemPackContentsVariant0 = String;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ItemDataItemPackContentsVariant1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    pub special: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ItemDataItemPackContentsVariant2 {
    pub item: String,
    pub quantity: i64,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ItemDataItemPackContents {
    Variant0(ItemDataItemPackContentsVariant0),
    Variant1(ItemDataItemPackContentsVariant1),
    Variant2(ItemDataItemPackContentsVariant2),
}
pub type ItemDataReqAttuneVariant0 = String;
pub type ItemDataReqAttuneVariant1 = bool;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ItemDataReqAttune {
    Variant0(ItemDataReqAttuneVariant0),
    Variant1(ItemDataReqAttuneVariant1),
}
pub type ItemDataReqAttuneAltVariant0 = String;
pub type ItemDataReqAttuneAltVariant1 = bool;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ItemDataReqAttuneAlt {
    Variant0(ItemDataReqAttuneAltVariant0),
    Variant1(ItemDataReqAttuneAltVariant1),
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(rename = "itemData")]
pub struct ItemData {
    #[doc = " Item ability score adjustments."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ability: Option<ItemDataAbility>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ac: Option<i64>,
    #[doc = " Free text field for homebrew use."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "acSpecial")]
    pub ac_special: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "additionalEntries")]
    pub additional_entries: Option<Vec<EntryJson>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "additionalSources")]
    pub additional_sources: Option<AdditionalSources>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<String>,
    #[doc = " Adventurers League item certificate ID; for use in homebrew."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "alCertificateId")]
    pub al_certificate_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ammoType")]
    pub ammo_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ammunition: Option<bool>,
    #[doc = " If the item's pack contents should be treated as one atomic unit, rather than handled as "]
    #[doc = " individual sub-items."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "atomicPackContents")]
    pub atomic_pack_contents: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "attachedSpells")]
    pub attached_spells: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub axe: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "baseItem")]
    pub base_item: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "bonusAbilityCheck")]
    pub bonus_ability_check: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "bonusAc")]
    pub bonus_ac: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "bonusProficiencyBonus")]
    pub bonus_proficiency_bonus: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "bonusSavingThrow")]
    pub bonus_saving_throw: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "bonusSpellAttack")]
    pub bonus_spell_attack: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "bonusSpellDamage")]
    pub bonus_spell_damage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "bonusSpellSaveDc")]
    pub bonus_spell_save_dc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "bonusWeapon")]
    pub bonus_weapon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "bonusWeaponAttack")]
    pub bonus_weapon_attack: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "bonusWeaponDamage")]
    pub bonus_weapon_damage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "capCargo")]
    pub cap_cargo: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "capPassenger")]
    pub cap_passenger: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "carryingCapacity")]
    pub carrying_capacity: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charges: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "conditionImmune")]
    pub condition_immune: Option<ConditionImmunityArray>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "containerCapacity")]
    pub container_capacity: Option<ItemDataContainerCapacity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crew: Option<i64>,
    #[doc = " For crews specified as an X-Y min-max."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "crewMax")]
    pub crew_max: Option<i64>,
    #[doc = " For crews specified as an X-Y min-max."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "crewMin")]
    pub crew_min: Option<i64>,
    #[doc = " ID of a value conversion table. Homebrew only."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "currencyConversion")]
    pub currency_conversion: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub curse: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "detail1")]
    pub detail_1: Option<String>,
    #[doc = " Maximum dexterity modifier for medium armor."]
    #[serde(default)]
    #[serde(rename = "dexterityMax")]
    pub dexterity_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dmg1")]
    pub dmg_1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dmg2")]
    pub dmg_2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dmgType")]
    pub dmg_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<EntryJson>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firearm: Option<bool>,
    #[doc = " Item can be used as a spellcasting focus"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus: Option<ItemDataFocus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "grantsProficiency")]
    pub grants_proficiency: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hasFluff")]
    pub has_fluff: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hasFluffImages")]
    pub has_fluff_images: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hasRefs")]
    pub has_refs: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub immune: Option<DamageImmunityArray>,
    #[doc = " This is required for itemGroup entries."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "lootTables")]
    pub loot_tables: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "otherSources")]
    pub other_sources: Option<OtherSources>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "packContents")]
    pub pack_contents: Option<Vec<ItemDataItemPackContents>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poison: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "poisonTypes")]
    pub poison_types: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
    #[doc = " \"None\" is for mundane items. \"Unknown (Magic)\" is for miscellaneous magical items. "]
    #[doc = " \"Unknown\" is for miscellaneous mundane items. \"Varies\" is for item groups."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rarity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recharge: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reload: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "reqAttune")]
    pub req_attune: Option<ItemDataReqAttune>,
    #[doc = " Used for filtering."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "reqAttuneAlt")]
    pub req_attune_alt: Option<ItemDataReqAttuneAlt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "reqAttuneAltTags")]
    pub req_attune_alt_tags: Option<ReqAttuneTags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "reqAttuneTags")]
    pub req_attune_tags: Option<ReqAttuneTags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resist: Option<DamageResistArray>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "scfType")]
    pub scf_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentient: Option<bool>,
    #[doc = " In copper pieces per 100 lbs per mile."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "shippingCost")]
    pub shipping_cost: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srd: Option<Srd>,
    #[doc = " Adds the italicized \"Staff\" text to the item info line (below the name)."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub staff: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stealth: Option<bool>,
    #[serde(default)]
    pub strength: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sword: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tattoo: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[doc = " In copper pieces per mile per passenger."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "travelCost")]
    pub travel_cost: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[doc = " In copper pieces."]
    #[serde(default)]
    pub value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "valueMult")]
    pub value_mult: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "vehAc")]
    pub veh_ac: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "vehDmgThresh")]
    pub veh_dmg_thresh: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "vehHp")]
    pub veh_hp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "vehSpeed")]
    pub veh_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulnerable: Option<DamageVulnerabilityArray>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weapon: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "weaponCategory")]
    pub weapon_category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "weightMult")]
    pub weight_mult: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "weightNote")]
    pub weight_note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wondrous: Option<bool>,
}
