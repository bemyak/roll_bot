use super::{Document, Entry, ValueExt};
use regex::{Captures, Regex};

pub trait Abbreviation: Entry {
	fn get_abbreviation(&self) -> Option<&str>;
	fn get_template(&self) -> Option<&str>;
	// Try to format given item using this template. If no template exists, returns the canonical name
	fn expand_abbreviation(&self, item: &Document) -> Option<String>;
}

impl Abbreviation for Document {
	fn get_abbreviation(&self) -> Option<&str> {
		self.get("abbreviation")?.as_str()
	}
	fn get_template(&self) -> Option<&str> {
		self.get("template")?.as_str()
	}
	fn expand_abbreviation(&self, item: &Document) -> Option<String> {
		let name = self.get_name()?;
		Some(
			get_templated(self, &name, item)
				.unwrap_or(name)
				.to_owned(),
		)
	}
}

fn get_templated<'a>(abbr: &'a Document, name: &str, item: &'a Document) -> Option<&'a str> {
	let prop_regex = Regex::new(r#"\{\{\s?prop_name\s?}}"#).unwrap();
	let item_regex = Regex::new(r#"\{\{\s?item\.(.+?)\s?}}"#).unwrap();
	let template = abbr.get("template")?.as_str()?;

	prop_regex.replace_all(template, name);
	item_regex.replace_all(template, |caps: &Captures| {
		if let Some(field) = caps.get(0) {
			if let Some(bson) = item.get(field.as_str()) {
				return bson.simple_format();
			}
		}
		"".to_string()
	});

	Some(template)
}
