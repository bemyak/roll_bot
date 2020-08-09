pub mod db;
pub mod roll;
pub mod spell;
pub mod telegram;
pub mod utils;

use std::fmt::Write;

use comfy_table::{presets::ASCII_NO_BORDERS, Cell, ContentArrangement, Row, Table};
use ejdb::bson::Document;
use ejdb::bson_crate::Bson;
use regex::Regex;

pub trait Entry {
    fn get_name(&self) -> Option<String>;
    fn get_source(&self) -> Option<String>;
    fn get_entries(&self) -> Option<Vec<String>>;
    // Very naive formatting
    fn format(&self) -> String;
}

impl Entry for Document {
    fn get_name(&self) -> Option<String> {
        self.get_str("name").map(str::to_owned).ok()
    }
    fn get_source(&self) -> Option<String> {
        let source = self.get_str("source").ok()?;
        let page = self.get_i64("page");
        let srd = self.get_bool("srd");

        let mut result = format!("Source: {}", source);

        if let Ok(page) = page {
            result.push_str(&format!(", page {}", page));
        }

        if let Ok(srd) = srd {
            if srd {
                result.push_str(". Available in the SRD.");
            }
        } else {
            result.push_str(".");
        }

        Some(result)
    }
    fn get_entries(&self) -> Option<Vec<String>> {
        let entries = self.get_array("entries").ok()?;

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

        if !result.is_empty() {
            Some(result)
        } else {
            None
        }
    }

    fn format(&self) -> String {
        let mut res = String::new();
        self.into_iter().for_each(|(k, v)| match k.as_ref() {
            "_id" => {}
            "name" => write!(&mut res, "*{}*\n\n", v).unwrap(),
            "entries" => {
                let s = match v {
                    Bson::Array(arr) => arr
                        .into_iter()
                        .map(|bs| simple_format(bs))
                        .collect::<Vec<String>>()
                        .join("\n\n"),
                    _ => simple_format(v),
                };
                write!(&mut res, "\n{}\n\n", s).unwrap()
            }
            _ => write!(&mut res, "*{}*: {}\n", k, simple_format(v)).unwrap(),
        });
        res
    }
}

fn format_entry(entry: &Bson) -> Option<String> {
    Some(match entry {
        Bson::String(entry) => entry.clone(),
        Bson::Document(entry) => match entry.get_str("type").ok()? {
            "list" => {
                let mut list_result = String::new();
                let items = get_string_array(entry, "items")?;

                for item in items {
                    list_result.push_str(&format!("• {}", item));
                }

                list_result
            }
            "entries" => {
                let name = entry.get_str("name").ok()?;
                let entries = get_string_array(entry, "entries")?;

                format!("*{}*: {}", name, entries.join("\n"))
            }
            "table" => {
                let mut table_result = String::new();
                let caption = entry.get_str("caption");

                if let Ok(caption) = caption {
                    table_result.push_str(&format!("*{}*\n", caption))
                }

                let mut table = Table::new();
                table
                    .load_preset(ASCII_NO_BORDERS)
                    .set_content_arrangement(ContentArrangement::Dynamic)
                    .set_table_width(35);

                if let Some(headers) = get_string_array(entry, "colLabels") {
                    table.set_header(Row::from(
                        headers
                            .iter()
                            .map(|header| Cell::new(header))
                            .collect::<Vec<_>>(),
                    ));
                }

                entry.get_array("rows").ok()?.iter().for_each(|row| {
                    if let Bson::Array(array) = row {
                        table.add_row(Row::from(
                            array
                                .iter()
                                .filter_map(|cell| format_entry(cell))
                                .map(|cell| demarkup(&cell))
                                .map(|cell| Cell::new(&cell))
                                .collect::<Vec<_>>(),
                        ));
                    }
                });

                table_result.push_str(&format!("```\n{}```", table));
                table_result
            }
            "cell" => {
                let roll = entry.get_document("roll").ok()?;

                let min = roll.get_i64("min");
                let max = roll.get_i64("max");
                let exact = roll.get_i64("exact");

                if let Ok(exact) = exact {
                    format!("{}", exact)
                } else if let (Ok(min), Ok(max)) = (min, max) {
                    format!("{}-{}", min, max)
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
    })
}

fn get_string_array(doc: &Document, key: &str) -> Option<Vec<String>> {
    let mut result = Vec::new();
    let items = doc.get_array(key).ok()?;
    for item in items {
        match item {
            Bson::String(entry) => {
                result.push(entry.clone());
            }
            _ => {
                continue;
            }
        }
    }
    Some(result)
}

fn simple_format(bs: &Bson) -> String {
    match bs {
        Bson::FloatingPoint(num) => format!("{}", num),
        Bson::String(s) => s.to_owned(),
        Bson::Array(arr) => arr
            .into_iter()
            .map(|bs| simple_format(bs))
            .collect::<Vec<String>>()
            .join(", "),
        Bson::Document(doc) => doc
            .into_iter()
            .map(|(k, v)| format!("{}: {}", k, simple_format(v)))
            .collect::<Vec<_>>()
            .join(", "),
        Bson::Boolean(b) => match b {
            true => "Yes".to_owned(),
            false => "No".to_owned(),
        },
        Bson::Null => "null".to_owned(),
        Bson::I32(num) => format!("{}", num),
        Bson::I64(num) => format!("{}", num),
        _ => panic!("Unknown type: {:?}", bs.element_type()),
    }
}

fn demarkup(s: &str) -> String {
    lazy_static! {
        static ref BOLD: Regex = Regex::new(r"(.*)\*(.+)\*(.*)").unwrap();
        static ref ITALIC: Regex = Regex::new(r"(.*)_(.+)_(.*)").unwrap();
        static ref STRIKE: Regex = Regex::new(r"(.*)\~(.+)\~(.*)").unwrap();
        static ref MONO: Regex = Regex::new(r"(.*)`(.+)`(.*)").unwrap();
        static ref ROLL: Regex = Regex::new(r"(.*)\s*\[(.+)\](.*)").unwrap();
    }

    let s = BOLD.replace_all(&s, "$1$2$3");
    let s = ITALIC.replace_all(&s, "$1$2$3");
    let s = STRIKE.replace_all(&s, "$1$2$3");
    let s = MONO.replace_all(&s, "$1$2$3");
    let s = ROLL.replace_all(&s, "$1$2$3");

    s.into()
}
