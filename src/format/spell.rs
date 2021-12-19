use super::{Capitalizable, Entry, FilterJoinable, Optionable};
use ejdb::bson::Document;
use ordinal::Ordinal;

pub trait Spell: Entry {
    fn get_meta(&self) -> Option<String>;
    fn get_casting_time(&self) -> Option<String>;
    fn get_range(&self) -> Option<String>;
    fn get_components(&self) -> Option<String>;
    fn get_duration(&self) -> Option<String>;

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

        times
            .iter()
            .map(|time| time.as_document())
            .flatten()
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

    fn format_spell(&self) -> Option<String> {
        let mut s = format!("*{}*", self.get_name()?);

        if let Some(meta) = self.get_meta() {
            s.push_str(&format!("\n_{}_", &meta));
        }

        s.push('\n');

        if let Some(casting_time) = self.get_casting_time() {
            s.push_str(&format!("\n*Casting time*: {}", &casting_time));
        }

        if let Some(range) = self.get_range() {
            s.push_str(&format!("\n*Range*: {}", &range));
        }

        if let Some(components) = self.get_components() {
            s.push_str(&format!("\n*Components*: {}", &components));
        }

        if let Some(duration) = self.get_duration() {
            s.push_str(&format!("\n*Duration*: {}", &duration));
        }

        if let Some(entries) = self.get_entries("entries") {
            s.push_str(&format!("\n\n{}", &entries.join("\n")));
        }

        if let Some(entries_high_level) = self.get_entries("entriesHigherLevel") {
            s.push_str(&format!("\n\n{}", &entries_high_level.join("\n")));
        }

        if let Some(source) = self.get_source() {
            s.push_str(&format!("\n\n_{}_", &source));
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
        ejdb::bson_crate::Bson::String(s) => Some(format!("M ({})", s)),
        ejdb::bson_crate::Bson::Boolean(_) => Some("M".to_string()),
        ejdb::bson_crate::Bson::Document(obj) => {
            let text = obj.get_str("text");
            match text {
                Ok(text) => Some(format!("M ({})", text)),
                Err(_) => Some("M".to_string()),
            }
        }
        _ => None,
    }
}

fn get_duration(duration: &Document) -> Option<String> {
    let type_ = duration.get_str("type").ok()?;
    let concentration = duration
        .get_bool("concentration")
        .ok()
        .map(|c| {
            if c {
                Some("concentration".to_string())
            } else {
                None
            }
        })
        .flatten();
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
