use super::{
	abbreviation::Abbreviation, cost_to_string, Capitalizable, Entry, EntryArrayUtils,
	FilterJoinable, Optionable,
};
use crate::DB;
use ejdb::bson::{Bson, Document};
use std::fmt::Write;

pub trait Item: Entry {
	// we need database to expand abbreviations
	fn format_item(&self) -> Option<String>;

	fn get_type(&self) -> Option<String>;
	fn get_attune(&self) -> Option<String>;
	fn get_value(&self) -> Option<i64>;
	fn get_carrying_capacity(&self) -> Option<i64>;
	fn get_ac(&self) -> Option<i64>;
	fn get_dmg1(&self) -> Option<String>;
	fn get_dmg_type(&self) -> Option<String>;
	fn get_speed(&self) -> Option<i64>;
	fn get_bonus_ac(&self) -> Option<String>;
	fn get_bonus_weapon_attack(&self) -> Option<String>;
	fn get_ammo_type(&self) -> Option<String>;
	fn get_properties(&self) -> Option<Vec<&str>>;
	fn get_weight(&self) -> Option<i64>;
	fn get_loot_tables(&self) -> Option<Vec<String>>;

	// For abbreviations

	// get_tags collects tags bellow
	fn get_tags(&self, type_: &Option<String>, abbr: &Option<Document>) -> Option<Vec<String>>;
	// simple, martial
	fn get_weapon_category(&self) -> Option<String>;
	fn get_tier(&self) -> Option<String>;
	fn get_rarity(&self) -> Option<String>;
}

impl Item for Document {
	fn format_item(&self) -> Option<String> {
		let mut s = format!("<b>{}</b>", self.get_name()?);

		let type_ = self.get_type();
		let (type_abbreviation, type_additional_abbreviation) = if let Some(type_) = &type_ {
			(
				DB.find_one_by("itemType", "abbreviation", type_)
					.ok()
					.flatten(),
				DB.find_one_by("itemTypeAdditionalEntries", "appliesTo", type_)
					.ok()
					.flatten(),
			)
		} else {
			(None, None)
		};

		let properties = self.get_properties();
		let property_abbreviations = if let Some(properties) = &properties {
			properties
				.iter()
				.filter_map(|p| DB.find_one_by("itemProperty", "abbreviation", p).ok())
				.collect::<Vec<_>>()
		} else {
			Vec::new()
		};

		let tags = self
			.get_tags(&type_, &type_abbreviation)
			.map(|t| t.join(", "));
		let tier = self.get_tier().map(|t| format!("{t} tier"));
		let rarity = self.get_rarity();

		let meta = vec![tags, tier, rarity]
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
			write!(s, "\n<b>Ammo Type</b>: {ammo_type}").ok()?;
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
			if let Some(entries) = t.and_then(|t| t.get_entries("entries")) {
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
	fn get_tags(&self, type_: &Option<String>, abbr: &Option<Document>) -> Option<Vec<String>> {
		let mut tags = vec![self.get_weapon_category()];

		let mut push_bool = |tag_name: &str| {
			if let Ok(tag) = self.get_bool(tag_name) {
				if tag {
					tags.push(Some(tag_name.to_string()))
				}
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

		tags.push(expand_type_abbreviation(self, type_, abbr));

		Some(tags.into_iter().flatten().collect())
	}
	fn get_rarity(&self) -> Option<String> {
		self.get_str("rarity").ok().map(|rarity| match rarity {
			"none" => "mundane".to_string(),
			"unknown" => "miscellaneous mundane".to_string(),
			"unknown (magic)" => "miscellaneous magical".to_string(),
			_ => rarity.to_string(),
		})
	}
	fn get_ac(&self) -> Option<i64> {
		self.get_i64("ac").ok()
	}
	fn get_type(&self) -> Option<String> {
		self.get_str("type").map(str::to_string).ok()
	}
	fn get_tier(&self) -> Option<String> {
		self.get_str("tier").map(str::to_string).ok()
	}
	fn get_carrying_capacity(&self) -> Option<i64> {
		self.get_i64("carryingCapacity").ok()
	}
	fn get_weight(&self) -> Option<i64> {
		self.get_i64("weight").ok()
	}
	fn get_properties(&self) -> Option<Vec<&str>> {
		self.get_array_of("property", Bson::as_str)?.into_option()
	}
	fn get_attune(&self) -> Option<String> {
		let base = "requires attunement".to_string();
		let b = self.get_bool("reqAttune").ok();
		if let Some(b) = b {
			if b {
				return Some(base);
			} else {
				return None;
			}
		}

		let s = self.get_str("reqAttune").ok()?;
		Some(base + " " + s)
	}
	fn get_dmg1(&self) -> Option<String> {
		self.get_str("dmg1").map(str::to_string).ok()
	}
	fn get_dmg_type(&self) -> Option<String> {
		self.get_str("dmgType")
			.map(|dmg_type| match dmg_type {
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
			.map(str::to_string)
			.ok()
	}
	fn get_speed(&self) -> Option<i64> {
		self.get_i64("speed").ok()
	}
	fn get_weapon_category(&self) -> Option<String> {
		self.get_str("weaponCategory").map(str::to_string).ok()
	}
	fn get_value(&self) -> Option<i64> {
		self.get_i64("value").ok()
	}
	fn get_loot_tables(&self) -> Option<Vec<String>> {
		let properties = self.get_array("lootTables").ok()?;
		let properties = properties
			.iter()
			.filter_map(Bson::as_str)
			.map(|property| property.to_string())
			.collect::<Vec<_>>();
		if properties.is_empty() {
			None
		} else {
			Some(properties)
		}
	}
	fn get_bonus_ac(&self) -> Option<String> {
		self.get_str("bonusAc").map(str::to_string).ok()
	}
	fn get_bonus_weapon_attack(&self) -> Option<String> {
		self.get_str("bonusWeaponAttack").map(str::to_string).ok()
	}
	fn get_ammo_type(&self) -> Option<String> {
		self.get_str("ammoType")
			.map(|at| format!("{{@item {at}}}"))
			.ok()
	}
}

fn expand_type_abbreviation(
	item: &Document,
	type_: &Option<String>,
	abbr: &Option<Document>,
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
