use crate::DB;

use super::{Capitalizable, Entry, FilterJoinable, Optionable};
use ejdb::bson::Document;
use ordinal::Ordinal;

use std::fmt::Write;

pub trait Spell: Entry {
	fn get_meta(&self) -> Option<String>;
	fn get_casting_time(&self) -> Option<String>;
	fn get_range(&self) -> Option<String>;
	fn get_components(&self) -> Option<String>;
	fn get_duration(&self) -> Option<String>;
	fn get_classes(&self) -> Option<Vec<String>>;

	fn format_spell(&self) -> Option<String>;
}

impl Spell for Document {
	fn get_meta(&self) -> Option<String> {
		let level = self.get_i64("level").ok()?;
		let level = Ordinal(level).to_string();

		let school = self.get_str("school");

		let (is_ritual, is_technomagic) = {
			let meta = self.get_document("meta");
			if let Ok(meta) = meta {
				let is_ritual = meta.get_bool("ritual").unwrap_or_default();
				let is_technomagic = meta.get_bool("ritual").unwrap_or_default();
				(is_ritual, is_technomagic)
			} else {
				(false, false)
			}
		};

		let mut result = format!("{level}-level");
		if let Ok(school) = school {
			let school = match school {
				"A" => "abjuration",
				"V" => "evocation",
				"E" => "enchantment",
				"I" => "illusion",
				"D" => "divination",
				"N" => "necromancy",
				"T" => "transmutation",
				"C" => "conjuration",
				"P" => "psionic",
				_ => school,
			};
			write!(result, " {school}").ok()?;
		}
		if is_ritual {
			result.push_str(" ritual");
		}
		if is_technomagic {
			result.push_str(" technomagic");
		}

		Some(result)
	}

	fn get_casting_time(&self) -> Option<String> {
		let times = self.get_array("time").ok()?;

		times
			.iter()
			.filter_map(|time| time.as_document())
			.filter_map(|time| {
				let number = time.get_i64("number").map(|number| number.to_string()).ok();
				let unit = time.get_str("unit").map(str::to_owned).ok();
				let condition = time.get_str("condition").map(str::to_owned).ok();

				vec![number, unit, condition].filter_join(" ")
			})
			.collect::<Vec<_>>()
			.join(", ")
			.into_option()
	}

	fn get_range(&self) -> Option<String> {
		let range = self.get_document("range").ok()?;
		let type_ = range.get_str("type").map(|type_| type_.to_string()).ok();
		let distance = get_distance(range);

		vec![distance, type_].filter_join(" ")
	}

	fn get_components(&self) -> Option<String> {
		let components = self.get_document("components").ok()?;

		let v = components.get_bool("v").unwrap_or(false);
		let v = if v { Some("V".to_string()) } else { None };

		let s = components.get_bool("s").unwrap_or(false);
		let s = if s { Some("S".to_string()) } else { None };

		let m = get_material(components);

		vec![v, s, m].filter_join(", ")
	}

	fn get_duration(&self) -> Option<String> {
		let durations = self.get_array("duration").ok()?;

		durations
			.iter()
			.filter_map(|bs| bs.as_document())
			.filter_map(get_duration)
			.collect::<Vec<_>>()
			.join(", ")
			.into_option()
	}

	fn get_classes(&self) -> Option<Vec<String>> {
		let search_name = format!("{} ({})", self.get_name()?, self.get_str("source").ok()?);
		let refs = DB.get_item("spell_sources", &search_name).ok()??;
		let classes = refs.get_array("class").ok()?.into_iter();
		let class_variants = refs
			.get_array("classVariant")
			.ok()
			.map(|class_variants| class_variants.into_iter())
			.unwrap_or_default();

		Some(
			classes
				.chain(class_variants)
				.filter_map(|class| {
					let doc = class.as_document()?;
					let class_name = doc.get_str("name").ok();
					let source = doc.get_str("definedInSource").ok();
					if let (Some(class_name), Some(source)) = (class_name, source) {
						Some(format!("{class_name} ({source} variant)"))
					} else {
						class_name.map(|class| class.to_string())
					}
				})
				.collect(),
		)
	}

	fn format_spell(&self) -> Option<String> {
		let mut s = format!("<b>{}</b>", self.get_name()?);

		if let Some(meta) = self.get_meta() {
			write!(s, "\n<i>{}</i>", &meta).ok()?;
		}

		s.push('\n');

		if let Some(casting_time) = self.get_casting_time() {
			write!(s, "\n<b>Casting time</b>: {}", &casting_time).ok()?;
		}

		if let Some(range) = self.get_range() {
			write!(s, "\n<b>Range</b>: {}", &range).ok()?;
		}

		if let Some(components) = self.get_components() {
			write!(s, "\n<b>Components</b>: {}", &components).ok()?;
		}

		if let Some(duration) = self.get_duration() {
			write!(s, "\n<b>Duration</b>: {}", &duration).ok()?;
		}

		if let Some(entries) = self.get_entries("entries") {
			write!(s, "\n\n{}", &entries.join("\n")).ok()?;
		}

		if let Some(entries_high_level) = self.get_entries("entriesHigherLevel") {
			write!(s, "\n\n{}", &entries_high_level.join("\n")).ok()?;
		}

		if let Some(classes) = self.get_classes() {
			write!(s, "\n\n<b>Classes</b>: {}", &classes.join(", ")).ok()?;
		}

		if let Some(source) = self.get_source() {
			write!(s, "\n\n<i>{}</i>", &source).ok()?;
		}

		Some(s)
	}
}

fn get_distance(range: &Document) -> Option<String> {
	let distance = range.get_document("distance").ok()?;
	let type_ = distance.get_str("type").map(|type_| type_.to_string()).ok();
	let amount = distance
		.get_i64("amount")
		.map(|amount| amount.to_string())
		.ok();

	vec![amount, type_].filter_join(" ")
}

fn get_material(components: &Document) -> Option<String> {
	let m = components.get("m")?;
	match m {
		ejdb::bson_crate::Bson::String(s) => Some(format!("M ({s})")),
		ejdb::bson_crate::Bson::Boolean(_) => Some("M".to_string()),
		ejdb::bson_crate::Bson::Document(obj) => {
			let text = obj.get_str("text");
			match text {
				Ok(text) => Some(format!("M ({text})")),
				Err(_) => Some("M".to_string()),
			}
		}
		_ => None,
	}
}

fn get_duration(duration: &Document) -> Option<String> {
	let type_ = duration.get_str("type").ok()?;
	let concentration = duration.get_bool("concentration").ok().and_then(|c| {
		if c {
			Some("concentration".to_string())
		} else {
			None
		}
	});
	let duration = duration
		.get_document("duration")
		.map(|duration| {
			let type_ = duration.get_str("type").map(|s| s.to_string()).ok();
			let amount = duration
				.get_i64("amount")
				.map(|amount| amount.to_string())
				.ok();
			let up_to = duration.get_bool("upTo").unwrap_or(false);
			let up_to = if up_to {
				Some("up to".to_string())
			} else {
				None
			};

			vec![up_to, amount, type_].filter_join(" ")
		})
		.ok()
		.flatten();

	let s = vec![concentration, duration].filter_join(", ");

	let result = match type_ {
		"timed" => s,
		_ => vec![Some(type_.to_string()), s].filter_join(", "),
	};

	result.map(|s| s.capitalize())
}
