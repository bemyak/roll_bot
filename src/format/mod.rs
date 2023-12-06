pub mod abbreviation;
// pub mod db;
pub mod item;
// pub mod monster;
pub mod roll;
// pub mod spell;
pub mod telegram;
pub mod utils;
// Not in use yet
// mod gen;

use std::fmt::Write;

use comfy_table::{presets::ASCII_NO_BORDERS, Cell, ContentArrangement, Row, Table};
use regex::Regex;
use serde_json::{Map, Value};

pub type Document = Map<String, Value>;

pub trait Entry {
	fn get_name(&self) -> Option<&str>;
	fn get_source(&self) -> Option<String>;
	fn get_entries(&self, key: &str) -> Option<Vec<String>>;
	fn get_named_entries(&self, key: &str) -> Option<String>;

	// Very naive formatting, mostly for debug
	fn format(&self) -> String;
}

impl Entry for Document {
	fn get_name(&self) -> Option<&str> {
		self.get("name")?.as_str()
	}
	fn get_source(&self) -> Option<String> {
		let source = self.get("source")?.as_str()?;
		let page = self.get("page").map(Value::as_u64).flatten();
		let srd = self.get("srd").map(Value::as_bool).flatten();

		let mut result = source.to_string();

		if let Some(page) = page {
			write!(result, ", page {page}").ok()?;
		}

		if let Some(srd) = srd {
			if srd {
				result.push_str(". Available in the SRD.");
			}
		} else {
			result.push('.');
		}

		Some(result)
	}
	fn get_entries(&self, key: &str) -> Option<Vec<String>> {
		let entries = self.get(key)?.as_array()?;

		let mut result = Vec::new();

		for entry in entries {
			if let Some(entry) = format_entry(entry) {
				result.push(entry);
			} else {
				error!(
					"Cannot properly format entries for \"{}\"",
					self.get_name().unwrap_or_default()
				);
			}
		}

		result.into_option()
	}
	fn get_named_entries(&self, key: &str) -> Option<String> {
		let entries = self.get_array_of(key, Value::as_object)?;
		entries
			.into_iter()
			.filter_map(|t| {
				let name = t
					.get("name")
					.map(Value::as_str)
					.flatten()
					.map(|s| format!("<b>{s}</b>:"));
				let entries = t.get_entries("entries").map(|entries| entries.join("\n"));
				vec![name, entries].filter_join(" ")
			})
			.collect::<Vec<_>>()
			.join("\n")
			.into_option()
	}
	fn format(&self) -> String {
		let mut res = String::new();
		self.into_iter().for_each(|(k, v)| match k.as_ref() {
			"_id" => {}
			"name" => write!(&mut res, "<b>{v}</b>\n\n").unwrap(),
			"entries" => {
				let s = match v {
					Value::Array(arr) => arr
						.iter()
						.map(Value::simple_format)
						.collect::<Vec<String>>()
						.join("\n\n"),
					_ => v.simple_format(),
				};
				write!(&mut res, "\n{s}\n\n").unwrap()
			}
			_ => writeln!(&mut res, "<b>{}</b>: {}", k, v.simple_format()).unwrap(),
		});
		res
	}
}

pub trait EntryUtils: Entry {
	fn get_string(&self, key: &str) -> Option<String>;
	fn get_object_str_fields(&self, key: &str) -> Option<Vec<(String, String)>>;
}
impl EntryUtils for Document {
	fn get_string(&self, key: &str) -> Option<String> {
		self.get(key)?.as_str().map(str::to_owned)
	}
	fn get_object_str_fields(&self, key: &str) -> Option<Vec<(String, String)>> {
		let doc = self.get(key)?.as_object()?;
		doc.into_iter()
			.filter_map(|(k, v)| {
				let v = v.as_str();
				v.map(|v| (k.to_string(), v.to_string()))
			})
			.collect::<Vec<_>>()
			.into_option()
	}
}

pub trait EntryArrayUtils<T: ?Sized> {
	fn get_array_of(&self, key: &str, f: fn(&Value) -> Option<&T>) -> Option<Vec<&T>>;
}
impl<T: ?Sized> EntryArrayUtils<T> for Document {
	fn get_array_of(&self, key: &str, f: fn(&Value) -> Option<&T>) -> Option<Vec<&T>> {
		let arr = self.get(key)?.as_array()?;
		let result = arr.iter().filter_map(f).collect::<Vec<_>>();
		result.into_option()
	}
}

fn format_entry(entry: &Value) -> Option<String> {
	match entry {
		Value::String(entry) => entry.clone(),
		Value::Object(entry) => match entry.get("type")?.as_str()? {
			"list" => {
				let items = entry.get("items")?.as_array()?;
				items
					.iter()
					.filter_map(format_entry)
					.map(|s| format!("\t• {s}"))
					.collect::<Vec<_>>()
					.join("\n")
			}
			"item" => {
				let name = entry.get("name")?.as_str()?;
				let entry = entry.get_string("entry").or_else(|| {
					entry
						.get("entries")
						.map(Value::as_array)
						.flatten()
						.map(|array| {
							array
								.iter()
								.filter_map(format_entry)
								.collect::<Vec<_>>()
								.join("\n")
						})
				})?;

				format!("{name}: {entry}")
			}
			"entries" => {
				let name = entry.get("name")?.as_str()?;
				let entries = entry.get_array_of("entries", Value::as_str)?;

				format!("<b>{}</b>: {}", name, entries.join("\n"))
			}
			"table" => {
				let mut table_result = String::new();
				let caption = entry.get("caption").map(Value::as_str).flatten();

				if let Some(caption) = caption {
					writeln!(table_result, "<b>{caption}</b>").ok()?
				}

				let mut table = Table::new();
				table
					.load_preset(ASCII_NO_BORDERS)
					.set_content_arrangement(ContentArrangement::Dynamic)
					.set_width(35);

				if let Some(headers) = entry.get_array_of("colLabels", Value::as_str) {
					table.set_header(Row::from(
						headers
							.iter()
							.map(|h| {
								let h = demarkup(h);
								Cell::new(h)
							})
							.collect::<Vec<_>>(),
					));
				}

				entry.get("rows")?.as_array()?.iter().for_each(|row| {
					if let Value::Array(array) = row {
						table.add_row(Row::from(
							array
								.iter()
								.filter_map(format_entry)
								.map(|cell| demarkup(&cell))
								.map(Cell::new)
								.collect::<Vec<_>>(),
						));
					}
				});

				write!(table_result, "<pre>\n{table}</pre>").ok()?;
				table_result
			}
			"cell" => {
				let roll = entry.get("roll")?.as_object()?;

				let min = roll.get("min").map(Value::as_i64).flatten();
				let max = roll.get("max").map(Value::as_i64).flatten();
				let exact = roll.get("exact").map(Value::as_i64).flatten();

				if let Some(exact) = exact {
					format!("{exact}")
				} else if let (Some(min), Some(max)) = (min, max) {
					format!("{min}-{max}")
				} else {
					return None;
				}
			}
			_ => {
				return None;
			}
		},
		_ => {
			return None;
		}
	}
	.into_option()
}

pub fn cost_to_string(cost: i64) -> String {
	let cooper = cost % 10;
	let silver = ((cost - cooper) / 10) % 10;
	let gold = (cost - silver - cooper) / 100;

	let cooper = if cooper == 0 {
		None
	} else {
		Some(format!("{cooper}cp"))
	};
	let silver = if silver == 0 {
		None
	} else {
		Some(format!("{silver}sp"))
	};
	let gold = if gold == 0 {
		None
	} else {
		Some(format!("{gold}gp"))
	};

	vec![gold, silver, cooper]
		.into_iter()
		.flatten()
		.collect::<Vec<_>>()
		.join(" ")
}

fn demarkup(s: &str) -> String {
	let tag: Regex = Regex::new(r"</?\S+>").unwrap();
	let s = tag.replace_all(s, "$1");
	s.into()
}

#[cfg(test)]
mod test {

	use super::*;

	#[test]
	fn test_cost_to_string() {
		assert_eq!(cost_to_string(1234567), "12345gp 6sp 7cp".to_string())
	}

	#[test]
	fn test_cost_to_string_gp() {
		assert_eq!(cost_to_string(1234500), "12345gp".to_string())
	}
}

pub trait FilterJoinable: IntoIterator {
	fn filter_join(self, sep: &str) -> Option<String>;
}

impl<T> FilterJoinable for Vec<Option<T>>
where
	T: ToString,
{
	fn filter_join(self, sep: &str) -> Option<String> {
		self.into_iter()
			.flatten()
			.map(|t| t.to_string())
			.collect::<Vec<_>>()
			.join(sep)
			.into_option()
	}
}

pub trait Capitalizable {
	fn capitalize(self) -> String;
}

impl Capitalizable for String {
	fn capitalize(self) -> String {
		let mut c = self.chars();
		match c.next() {
			None => String::new(),
			Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
		}
	}
}

impl Capitalizable for &str {
	fn capitalize(self) -> String {
		let mut c = self.chars();
		match c.next() {
			None => String::new(),
			Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
		}
	}
}

pub trait Optionable: Sized {
	fn into_option(self) -> Option<Self>;
}

impl Optionable for String {
	fn into_option(self) -> Option<Self> {
		if self.is_empty() {
			None
		} else {
			Some(self)
		}
	}
}
impl Optionable for &str {
	fn into_option(self) -> Option<Self> {
		if self.is_empty() {
			None
		} else {
			Some(self)
		}
	}
}
impl<T> Optionable for Vec<T> {
	fn into_option(self) -> Option<Self> {
		if self.is_empty() {
			None
		} else {
			Some(self)
		}
	}
}

pub trait ValueExt {
	fn into_object(self) -> Option<Document>;
	fn simple_format(self: &Self) -> String;
}

impl ValueExt for Value {
	fn into_object(self) -> Option<Document> {
		match self {
			Value::Object(obj) => Some(obj),
			_ => None,
		}
	}

	fn simple_format(self: &Self) -> String {
		match self {
			Value::String(s) => s.to_owned(),
			Value::Array(arr) => arr
				.iter()
				.map(Self::simple_format)
				.collect::<Vec<String>>()
				.join(", "),
			Value::Object(doc) => doc
				.iter()
				.map(|(k, v)| format!("{}: {}", k, v.simple_format()))
				.collect::<Vec<_>>()
				.join(", "),
			Value::Bool(b) => match b {
				true => "Yes".to_owned(),
				false => "No".to_owned(),
			},
			Value::Null => "null".to_owned(),
			Value::Number(num) => num.to_string(),
			// Bson::I32(num) => format!("{num}"),
			// Bson::I64(num) => format!("{num}"),
			// _ => panic!("Unknown type: {:?}", bs.element_type()),
		}
	}
}
