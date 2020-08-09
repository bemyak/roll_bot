use super::Entry;
use ejdb::bson::Document;
use ordinal::Ordinal;
use std::convert::identity;

pub trait Spell: Entry {
    fn get_meta(&self) -> Option<String>;
    fn get_casting_time(&self) -> Option<String>;
    fn get_range(&self) -> Option<String>;
    fn get_components(&self) -> Option<String>;
    fn get_duration(&self) -> Option<String>;
    fn get_at_higher_levels(&self) -> Option<Vec<String>>;

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

        let mut result = format!("{}-level", level);
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
            result.push_str(&format!(" {}", school));
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

        let s = times
            .into_iter()
            .map(|time| time.as_document())
            .filter_map(identity)
            .map(|time| {
                let number = time.get_i64("number").map(|number| number.to_string()).ok();
                let unit = time.get_str("unit").map(str::to_owned).ok();
                let condition = time.get_str("condition").map(str::to_owned).ok();

                vec![number, unit, condition]
                    .into_iter()
                    .filter_map(identity)
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .collect::<Vec<_>>()
            .join(", ");

        if s.is_empty() {
            None
        } else {
            Some(s)
        }
    }

    fn get_range(&self) -> Option<String> {
        None
    }

    fn get_components(&self) -> Option<String> {
        None
    }

    fn get_duration(&self) -> Option<String> {
        None
    }

    fn get_at_higher_levels(&self) -> Option<Vec<String>> {
        None
    }

    fn format_spell(&self) -> Option<String> {
        let mut s = format!("*{}*", self.get_name()?);

        if let Some(meta) = self.get_meta() {
            s.push_str(&format!("\n_{}_", &meta));
        }

        s.push_str("\n");

        if let Some(casting_time) = self.get_casting_time() {
            s.push_str(&format!("\n*Casting time*: {}", &casting_time));
        }

        if let Some(entries) = self.get_entries() {
            s.push_str(&format!("\n\n{}", &entries.join("\n")));
        }

        if let Some(source) = self.get_source() {
            s.push_str(&format!("\n\n_{}_", &source));
        }

        Some(s)
    }
}
