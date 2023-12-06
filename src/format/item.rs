use super::{
	abbreviation::Abbreviation, cost_to_string, Capitalizable, Document, Entry, EntryArrayUtils,
	FilterJoinable, Optionable, ValueExt,
};
use crate::DB;
use async_trait::async_trait;
use serde_json::Value;
use std::fmt::Write;

#[async_trait]
pub trait Item: Entry {
	// we need database to expand abbreviations
	async fn format_item(&self) -> Option<String>;

	fn get_type(&self) -> Option<&str>;
	fn get_attune(&self) -> Option<String>;
	fn get_value(&self) -> Option<i64>;
	fn get_carrying_capacity(&self) -> Option<i64>;
	fn get_ac(&self) -> Option<i64>;
	fn get_dmg1(&self) -> Option<&str>;
	fn get_dmg_type(&self) -> Option<&str>;
	fn get_speed(&self) -> Option<i64>;
	fn get_bonus_ac(&self) -> Option<&str>;
	fn get_bonus_weapon_attack(&self) -> Option<&str>;
	fn get_ammo_type(&self) -> Option<&str>;
	fn get_properties(&self) -> Option<Vec<&str>>;
	fn get_weight(&self) -> Option<i64>;
	fn get_loot_tables(&self) -> Option<Vec<&str>>;

	// For abbreviations

	// get_tags collects tags bellow
	fn get_tags(&self, type_: Option<&str>, abbr: Option<&Document>) -> Option<Vec<String>>;
	// simple, martial
	fn get_weapon_category(&self) -> Option<&str>;
	fn get_tier(&self) -> Option<&str>;
	fn get_rarity(&self) -> Option<&str>;
}

#[async_trait]
impl Item for Document {
	async fn format_item(&self) -> Option<String> {
		let mut s = format!("<b>{}</b>", self.get_name()?);

		let type_ = self.get_type();
		let (type_abbreviation, type_additional_abbreviation) = if let Some(type_) = &type_ {
			(
				DB.find_one_by("itemType", "abbreviation", type_)
					.await
					.ok()
					.map(Value::into_object)
					.flatten(),
				DB.find_one_by("itemTypeAdditionalEntries", "appliesTo", type_)
					.await
					.ok()
					.map(Value::into_object)
					.flatten(),
			)
		} else {
			(None, None)
		};

		let property_abbreviations = {
			let mut property_abbreviations = vec![];
			if let Some(properties) = &self.get_properties() {
				for property in properties {
					if let Some(full_property) = DB
						.find_one_by("itemProperty", "abbreviation", property)
						.await
						.ok()
						.map(Value::into_object)
						.flatten()
					{
						property_abbreviations.push(full_property);
					}
				}
			}
			property_abbreviations
		};

		let tags = self
			.get_tags(type_, type_abbreviation.as_ref())
			.map(|t| t.join(", "));
		let tier = self.get_tier().map(|t| format!("{t} tier"));
		let rarity = self.get_rarity();

		let meta = vec![
			tags.as_ref().map(String::as_str),
			tier.as_ref().map(String::as_str),
			rarity,
		]
		.filter_join(", ")
		.map(|s| s.capitalize());
		if let Some(meta) = meta {
			write!(s, "\n<i>{meta}</i>").ok()?;
		}

		if let Some(attune) = self.get_attune() {
			write!(s, "\n<i>{}</i>", attune.capitalize()).ok()?;
		}
		if let Some(value) = self.get_value() {
			write!(s, "\n\n<b>Cost</b>: {}", cost_to_string(value)).ok()?;
		}
		if let Some(carrying_capacity) = self.get_carrying_capacity() {
			write!(s, "\n<b>Carrying capacity</b>: {carrying_capacity} lb.").ok()?;
		}
		if let Some(ac) = self.get_ac() {
			write!(s, "\n<b>AC</b>: {ac}").ok()?;
		}
		if let Some(dmg1) = self.get_dmg1() {
			write!(s, "\n<b>Damage</b>: {dmg1}").ok()?;
			if let Some(dmg_type) = self.get_dmg_type() {
				write!(s, " {dmg_type}").ok()?
			}
		} else if let Some(dmg_type) = self.get_dmg_type() {
			write!(s, "\n<b>Damage type</b>: {dmg_type}").ok()?
		}
		if let Some(speed) = self.get_speed() {
			write!(s, "\n<b>Speed</b>: {speed}").ok()?;
		}

		if let Some(weight) = self.get_weight() {
			write!(s, "\n<b>Weight</b>: {weight} lb").ok()?;
		}

		if let Some(ammo_type) = self.get_ammo_type() {
			write!(s, "\n<b>Ammo Type</b>: {{@item {ammo_type}}}").ok()?;
		}

		if let Some(bonus_ac) = self.get_bonus_ac() {
			write!(s, "\n<b>AC Bonus</b>: {bonus_ac}").ok()?;
		}
		if let Some(bonus_weapon_attack) = self.get_bonus_weapon_attack() {
			write!(s, "\n<b>Attack Bonus</b>: {bonus_weapon_attack}").ok()?;
		}

		if let Some(entries) = self.get_entries("entries") {
			write!(s, "\n\n{}", &entries.join("\n")).ok()?;
		}
		if let Some(entries) = type_abbreviation.and_then(|t| t.get_entries("entries")) {
			write!(s, "\n\n{}", &entries.join("\n")).ok()?;
		}
		if let Some(entries) = type_additional_abbreviation.and_then(|t| t.get_entries("entries")) {
			write!(s, "\n\n{}", &entries.join("\n")).ok()?;
		}
		for t in property_abbreviations {
			if let Some(entries) = t.get_entries("entries") {
				write!(s, "\n\n{}", &entries.join("\n")).ok()?;
			}
		}

		if let Some(loot_tables) = self.get_loot_tables() {
			write!(s, "\n\n<b>Loot tables</b>: {}", loot_tables.join(" ")).ok()?;
		}
		if let Some(source) = self.get_source() {
			write!(s, "\n\n<i>{source}</i>").ok()?;
		}

		Some(s)
	}
	fn get_tags(&self, type_: Option<&str>, abbr: Option<&Document>) -> Option<Vec<String>> {
		let mut tags = vec![self.get_weapon_category()];

		let mut push_bool = |tag_name: &'static str| {
			if let Some(true) = self.get(tag_name).map(Value::as_bool).flatten() {
				tags.push(Some(tag_name))
			}
		};

		push_bool("ammunition");
		push_bool("axe");
		push_bool("sword");
		push_bool("firearm");
		push_bool("staff");
		push_bool("weapon");
		push_bool("wondrous");
		push_bool("tattoo");
		push_bool("sentient");
		push_bool("poison");

		let expanded = expand_type_abbreviation(self, type_, abbr);
		tags.push(expanded.as_ref().map(String::as_str));

		// TODO: Do not copy?
		Some(
			tags.into_iter()
				.filter_map(|s| s)
				.map(|s| s.to_owned())
				.collect(),
		)
	}
	fn get_rarity(&self) -> Option<&str> {
		let rarity = self.get("rarity")?.as_str()?;
		Some(match rarity {
			"none" => "mundane",
			"unknown" => "miscellaneous mundane",
			"unknown (magic)" => "miscellaneous magical",
			_ => rarity,
		})
	}
	fn get_ac(&self) -> Option<i64> {
		self.get("ac")?.as_i64()
	}
	fn get_type(&self) -> Option<&str> {
		self.get("type")?.as_str()
	}
	fn get_tier(&self) -> Option<&str> {
		self.get("tier")?.as_str()
	}
	fn get_carrying_capacity(&self) -> Option<i64> {
		self.get("carryingCapacity")?.as_i64()
	}
	fn get_weight(&self) -> Option<i64> {
		self.get("weight")?.as_i64()
	}
	fn get_properties(&self) -> Option<Vec<&str>> {
		self.get_array_of("property", Value::as_str)?.into_option()
	}
	fn get_attune(&self) -> Option<String> {
		let base = "requires attunement".to_string();
		let b = self.get("reqAttune").map(Value::as_bool).flatten();
		if let Some(b) = b {
			if b {
				return Some(base);
			} else {
				return None;
			}
		}

		let s = self.get("reqAttune")?.as_str()?;
		Some(base + " " + s)
	}
	fn get_dmg1(&self) -> Option<&str> {
		self.get("dmg1")?.as_str()
	}
	fn get_dmg_type(&self) -> Option<&str> {
		let dmg_type = self.get("dmgType")?.as_str()?;
		Some(match dmg_type {
			"A" => "acid",
			"B" => "bludgeoning",
			"C" => "cold",
			"F" => "fire",
			"O" => "force",
			"L" => "lightning",
			"N" => "necrotic",
			"P" => "piercing",
			"I" => "poison",
			"Y" => "psychic",
			"R" => "radiant",
			"S" => "slashing",
			"T" => "thunder",
			_ => dmg_type,
		})
	}
	fn get_speed(&self) -> Option<i64> {
		self.get("speed")?.as_i64()
	}
	fn get_weapon_category(&self) -> Option<&str> {
		self.get("weaponCategory")?.as_str()
	}
	fn get_value(&self) -> Option<i64> {
		self.get("value")?.as_i64()
	}
	fn get_loot_tables(&self) -> Option<Vec<&str>> {
		let properties = self.get("lootTables")?.as_array()?;
		let properties = properties
			.iter()
			.filter_map(Value::as_str)
			.collect::<Vec<_>>();
		if properties.is_empty() {
			None
		} else {
			Some(properties)
		}
	}
	fn get_bonus_ac(&self) -> Option<&str> {
		self.get("bonusAc")?.as_str()
	}
	fn get_bonus_weapon_attack(&self) -> Option<&str> {
		self.get("bonusWeaponAttack")?.as_str()
	}
	fn get_ammo_type(&self) -> Option<&str> {
		self.get("ammoType")?.as_str()
	}
}

fn expand_type_abbreviation<'a>(
	item: &'a Document,
	type_: Option<&'a str>,
	abbr: Option<&'a Document>,
) -> Option<String> {
	let type_ = type_.as_ref()?;
	if let Some(abbr) = abbr {
		if let Some(expanded) = abbr.expand_abbreviation(item) {
			return Some(expanded);
		}
	}
	let fallback = match type_.as_ref() {
		"A" => "ammunition",
		"AF" => "ammunition",
		"AT" => "artisan's tools",
		"EM" => "eldritch machine",
		"EXP" => "explosive",
		"FD" => "food and drink",
		"G" => "adventuring gear",
		"GS" => "gaming set",
		"HA" => "heavy armor",
		"INS" => "instrument",
		"LA" => "light armor",
		"M" => "melee weapon",
		"MA" => "medium armor",
		"MNT" => "mount",
		"MR" => "master rune",
		"GV" => "generic variant",
		"P" => "potion",
		"R" => "ranged weapon",
		"RD" => "rod",
		"RG" => "ring",
		"S" => "shield",
		"SC" => "scroll",
		"SCF" => "spellcasting focus",
		"OTH" => "other",
		"T" => "tools",
		"TAH" => "tack and harness",
		"TG" => "trade good",
		"$" => "treasure",
		"VEH" => "vehicle (land)",
		"SHP" => "vehicle (water)",
		"AIR" => "vehicle (air)",
		"WD" => "wand",
		_ => type_,
	};
	Some(fallback.to_string())
}
