use super::Entry;
use ejdb::bson::Document;
use ordinal::Ordinal;

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
        let level = self.get_i32("level").ok()?;
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
        None
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
        Some(format!(
            "*{}*\n\n{}",
            self.get_name()?,
            self.get_entries()?.join("\n")
        ))
    }
}
