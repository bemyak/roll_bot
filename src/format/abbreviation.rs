use super::{simple_format, Entry};
use ejdb::bson::Document;
use regex::{Captures, Regex};

pub trait Abbreviation: Entry {
    fn get_abbreviation(&self) -> Option<String>;
    fn get_template(&self) -> Option<String>;
    // Try to format given item using this template. If no template exists, returns the canonical name
    fn expand_abbreviation(&self, item: &Document) -> Option<String>;
}

impl Abbreviation for Document {
    fn get_abbreviation(&self) -> Option<String> {
        self.get_str("abbreviation").map(|s| s.to_string()).ok()
    }
    fn get_template(&self) -> Option<String> {
        self.get_str("template").map(|s| s.to_string()).ok()
    }
    fn expand_abbreviation(&self, item: &Document) -> Option<String> {
        let name = self.get_name()?;
        Some(get_templated(self, &name, item).unwrap_or(name))
    }
}

fn get_templated(abbr: &Document, name: &str, item: &Document) -> Option<String> {
    let prop_regex = Regex::new(r"{{\s?prop_name\s?}}").unwrap();
    let item_regex = Regex::new(r"{{\s?item\.(.+?)\s?}}").unwrap();
    let template = abbr.get_str("template").ok()?;

    prop_regex.replace_all(template, name);
    item_regex.replace_all(template, |caps: &Captures| {
        if let Some(field) = caps.get(0) {
            if let Some(bson) = item.get(field.as_str()) {
                return simple_format(bson);
            }
        }

        return "".to_string();
    });

    Some(template.to_string())
}
