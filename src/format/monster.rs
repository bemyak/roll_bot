use super::{Capitalizable, Entry, EntryArrayUtils, EntryUtils, FilterJoinable, Optionable};
use ejdb::bson::{Bson, Document};
use ordinal::Ordinal;

pub trait Monster: Entry {
    fn format_monster(&self) -> Option<String>;
}

impl Monster for Document {
    fn format_monster(&self) -> Option<String> {
        let name = self.get_short_name().or(self.get_name())?;
        let mut result = format!("*{}*", name);

        let meta = vec![
            self.get_level(),
            self.get_size(),
            self.get_size_note(),
            self.get_type(),
            self.get_alignment(),
        ]
        .filter_join(", ");
        if let Some(meta) = meta {
            result.push_str(&format!("\n_{}_", meta));
        }

        todo!()
    }
}

trait MonsterPrivate: Monster {
    fn get_short_name(&self) -> Option<String>;
    fn get_level(&self) -> Option<String>;
    fn get_size(&self) -> Option<String>;
    fn get_size_note(&self) -> Option<String>;
    fn get_type(&self) -> Option<String>;
    fn get_alignment(&self) -> Option<String>;
    fn get_ac(&self) -> Option<String>;
    fn get_hp(&self) -> Option<String>;
    fn get_speed(&self) -> Option<String>;
    fn get_str(&self) -> Option<String>;
    fn get_dex(&self) -> Option<String>;
    fn get_con(&self) -> Option<String>;
    fn get_int(&self) -> Option<String>;
    fn get_wis(&self) -> Option<String>;
    fn get_cha(&self) -> Option<String>;
    fn get_save(&self) -> Option<String>;
    fn get_skill(&self) -> Option<String>;
    fn get_senses(&self) -> Option<String>;
    fn get_passive(&self) -> Option<String>;
    fn get_languages(&self) -> Option<String>;
    fn get_cr(&self) -> Option<String>;
    fn get_vulnerable(&self) -> Option<String>;
    fn get_resist(&self) -> Option<String>;
    fn get_immune(&self) -> Option<String>;
    fn get_condition_immune(&self) -> Option<String>;
    fn get_spellcasting(&self) -> Option<String>;
    fn get_trait(&self) -> Option<String>;
    fn get_action(&self) -> Option<String>;
    fn get_reaction(&self) -> Option<String>;
    fn get_legendary_group(&self) -> Option<String>;
    fn get_legendary_actions_num(&self) -> Option<i64>;
    fn get_legendary_header(&self) -> Option<Vec<String>>;
    fn get_legendary(&self) -> Option<String>;
}

impl MonsterPrivate for Document {
    fn get_short_name(&self) -> Option<String> {
        self.get_str("shortName")
            .ok()
            .map(str::to_string)
            .or(self.get_name())
    }
    fn get_level(&self) -> Option<String> {
        self.get_i64("level")
            .ok()
            .map(|l| format!("{} level", Ordinal(l)))
    }
    fn get_size(&self) -> Option<String> {
        self.get_str("size")
            .ok()
            .map(size_to_string)
            .map(str::to_string)
    }
    fn get_size_note(&self) -> Option<String> {
        self.get_str("sizeNote").map(str::to_string).ok()
    }
    fn get_type(&self) -> Option<String> {
        let type_doc = self.get_document("type").ok()?;
        let type_ = type_doc.get_str("type").ok()?;
        let swarm_size = type_doc.get_str("swarmSize").ok();
        let mut type_ = match swarm_size {
            Some(swarm_size) => format!("swarm of {} {}", size_to_string(swarm_size), type_),
            None => type_.to_string(),
        };

        let tags = type_doc.get_array_of("tags", Bson::as_str);
        match tags {
            Some(tags) => {
                type_.push_str(&format!(" ({})", tags.join(", ")));
            }
            None => {}
        };

        Some(type_)
    }
    fn get_alignment(&self) -> Option<String> {
        let arr = self.get_array("alignment").ok()?;
        let result = arr
            .into_iter()
            .filter_map(|element| match element {
                Bson::String(str) => Some(alignment_to_string(str).to_string()),
                Bson::Document(doc) => match doc.get("special") {
                    Some(special) => Some(special.to_string()),
                    None => doc.format_alignment_doc(),
                },
                _ => None,
            })
            .collect::<Vec<_>>()
            .join(" ");
        result.to_option()
    }
    fn get_ac(&self) -> Option<String> {
        let acs = self.get_array_of("ac", Bson::as_document)?;
        acs.into_iter()
            .filter_map(|doc| doc.format_ac_doc())
            .collect::<Vec<_>>()
            .join(" or ")
            .to_option()
    }
    fn get_hp(&self) -> Option<String> {
        let hp = self.get_document("hp").ok()?;
        let special = hp.get_string("special");
        if special.is_some() {
            return special;
        }
        let avg = hp.get_i64("average").ok().map(|i| i.to_string());
        let formula = hp.get_str("formula").ok().map(|s| format!("({})", s));
        vec![avg, formula].filter_join(" ")
    }
    fn get_speed(&self) -> Option<String> {
        match self.get("speed")? {
            Bson::Document(doc) => vec!["walk", "borrow", "climb", "fly", "swim"]
                .into_iter()
                .filter_map(|key| match doc.get(key) {
                    Some(value) => match value {
                        Bson::Document(doc) => doc.format_speed_val(),
                        Bson::I64(i) => Some(i.to_string()),
                        _ => None,
                    },
                    None => None,
                })
                .collect::<Vec<_>>()
                .join(", ")
                .to_option(),
            Bson::String(s) => Some(s.to_string()),
            Bson::I64(i) => Some(i.to_string()),
            _ => None,
        }
    }
    fn get_str(&self) -> Option<String> {
        self.get_stat("str")
    }
    fn get_dex(&self) -> Option<String> {
        self.get_stat("dex")
    }
    fn get_con(&self) -> Option<String> {
        self.get_stat("con")
    }
    fn get_int(&self) -> Option<String> {
        self.get_stat("int")
    }
    fn get_wis(&self) -> Option<String> {
        self.get_stat("wis")
    }
    fn get_cha(&self) -> Option<String> {
        self.get_stat("cha")
    }
    fn get_save(&self) -> Option<String> {
        let fields = self.get_object_str_fields("save")?;
        fields
            .into_iter()
            .map(|(k, v)| format!("{} {}", k, v).capitalize())
            .collect::<Vec<_>>()
            .join(", ")
            .to_option()
    }
    fn get_skill(&self) -> Option<String> {
        let fields = self.get_object_str_fields("skill")?;
        fields
            .into_iter()
            .map(|(k, v)| format!("{} {}", k, v).capitalize())
            .collect::<Vec<_>>()
            .join(", ")
            .to_option()
    }
    fn get_senses(&self) -> Option<String> {
        self.get_array_of("senses", Bson::as_str)
            .map(|arr| arr.join(", "))
    }
    fn get_passive(&self) -> Option<String> {
        self.get_i64("passive").map(|i| i.to_string()).ok()
    }
    fn get_languages(&self) -> Option<String> {
        self.get_array_of("languages", Bson::as_str)
            .map(|arr| arr.join(", "))
    }
    fn get_cr(&self) -> Option<String> {
        let cr = self.get("cr")?;
        match cr {
            Bson::String(s) => Some(s.to_string()),
            Bson::Document(doc) => {
                let cr = doc.get_string("cr");
                let lair = doc.get_str("lair").map(|s| format!("{} in lair", s)).ok();
                let coven = doc.get_str("coven").map(|s| format!("{} in coven", s)).ok();
                vec![cr, lair, coven].filter_join(" or ")
            }
            _ => None,
        }
    }
    fn get_vulnerable(&self) -> Option<String> {
        self.format_damage_property("vulnerable")
    }
    fn get_resist(&self) -> Option<String> {
        self.format_damage_property("resist")
    }
    fn get_immune(&self) -> Option<String> {
        self.format_damage_property("immune")
    }
    fn get_condition_immune(&self) -> Option<String> {
        self.format_damage_property("conditionImmune")
    }
    fn get_spellcasting(&self) -> Option<String> {
        let name = self.get_str("name").map(|s| format!("*{}*", s)).ok();
        let header = self.get_array_of("headerEntries", Bson::as_str);
        let footer = self.get_array_of("footerEntries", Bson::as_str);
        let at_will = self.get_array_of("will", Bson::as_str);
        let ritual = self.get_array_of("ritual", Bson::as_str);
        let rest = self.format_spell_frequency("rest");
        let daily = self.format_spell_frequency("daily");
        let weekly = self.format_spell_frequency("weekly");
        let spells = self.format_spells("spells");

        let mut result = String::new();

        if let Some(name) = name {
            result.push_str(&format!("*{}*: ", name));
        }
        if let Some(header) = header {
            result.push_str(&header.join("\n"));
        }
        if let Some(at_will) = at_will {
            result.push_str(&format!("\nAt will: {}", at_will.join(", ")));
        }
        if let Some(daily) = daily {
            result.push_str(&format!("\n{}", daily.join("\n")));
        }
        if let Some(rest) = rest {
            result.push_str(&format!("\n{}", rest.join("\n")));
        }
        if let Some(weekly) = weekly {
            result.push_str(&format!("\n{}", weekly.join("\n")));
        }
        if let Some(ritual) = ritual {
            result.push_str(&format!("\nRituals: {}", ritual.join(", ")));
        }
        if let Some(spells) = spells {
            result.push_str(&format!("\n{}", spells.join("\n")));
        }
        if let Some(footer) = footer {
            result.push_str(&format!("\n{}", footer.join("\n")));
        }

        Some(result)
    }
    fn get_trait(&self) -> Option<String> {
        self.get_named_entries("trait")
    }
    fn get_action(&self) -> Option<String> {
        self.get_named_entries("action")
    }
    fn get_reaction(&self) -> Option<String> {
        self.get_named_entries("reaction")
    }
    fn get_legendary_group(&self) -> Option<String> {
        self.get_string("legendaryGroup")
    }
    fn get_legendary_actions_num(&self) -> Option<i64> {
        self.get_i64("legendaryActions").ok()
    }
    fn get_legendary_header(&self) -> Option<Vec<String>> {
        self.get_entries("legendaryHeader")
    }
    fn get_legendary(&self) -> Option<String> {
        self.get_named_entries("legendary")
    }
}
trait MonsterUtils: Monster {
    fn format_alignment_doc(&self) -> Option<String>;
    fn format_ac_doc(&self) -> Option<String>;
    fn format_speed_val(&self) -> Option<String>;
    // immune, resist, vulnerability, conditionImmune
    fn format_damage_property(&self, key: &str) -> Option<String>;
    fn format_spell_frequency(&self, key: &str) -> Option<Vec<String>>;
    fn format_spells(&self, key: &str) -> Option<Vec<String>>;
    fn get_stat(&self, stat: &str) -> Option<String>;
}

impl MonsterUtils for Document {
    fn format_alignment_doc(&self) -> Option<String> {
        let alignment = self.get_alignment();
        let chance = self
            .get_i64("chance")
            .ok()
            .map(|i| format!("({}% chance)", i));
        let note = self.get_str("note").ok().map(str::to_string);
        vec![alignment, chance, note].filter_join(" ")
    }
    fn format_ac_doc(&self) -> Option<String> {
        let ac = self.get_i64("ac").ok().map(|i| i.to_string());
        let from = self
            .get_array_of("from", Bson::as_str)
            .map(|arr| format!("({})", arr.join(", ")));
        let condition = self.get_string("condition");
        vec![ac, from, condition].filter_join(" ")
    }
    fn format_speed_val(&self) -> Option<String> {
        let number = self.get_i64("number").ok()?;
        let condition = self.get_str("condition").ok()?;
        Some(format!("{} {}", number, condition))
    }
    fn format_damage_property(&self, key: &str) -> Option<String> {
        let arr = self.get_array(key).ok()?;
        arr.into_iter()
            .filter_map(|bs| match bs {
                Bson::String(s) => Some(s.to_string()),
                Bson::Document(doc) => match doc.get_str("special") {
                    Ok(s) => Some(s.to_string()),
                    Err(_) => {
                        let vulnerable = doc.format_damage_property(key);
                        let pre_note = doc.get_string("preNote");
                        let note = doc.get_string("note");
                        vec![pre_note, vulnerable, note].filter_join(" ")
                    }
                },
                _ => None,
            })
            .collect::<Vec<_>>()
            .join(", ")
            .to_option()
    }
    fn format_spell_frequency(&self, key: &str) -> Option<Vec<String>> {
        let doc = self.get_document(key).ok()?;
        doc.keys()
            .into_iter()
            .filter_map(|k| {
                doc.get_array_of(k, Bson::as_str).map(|spells| {
                    let spells = spells.join("\n");
                    if k.ends_with("e") {
                        let lvl = k.replace("e", "");
                        format!("{}/{} each: {}", lvl, key, spells)
                    } else {
                        format!("{}/{}: {}", k, key, spells)
                    }
                })
            })
            .collect::<Vec<_>>()
            .to_option()
    }
    fn format_spells(&self, key: &str) -> Option<Vec<String>> {
        let doc = self.get_document(key).ok()?;
        doc.into_iter()
            .filter_map(|(k, v)| {
                let k = k.parse::<i64>().ok();
                let v = v.as_document();
                match (k, v) {
                    (Some(k), Some(v)) => Some((k, v)),
                    _ => None,
                }
            })
            .filter_map(|(k, v)| {
                let lower = v.get_i64("lower").ok();
                let spells = v.get_array_of("spells", Bson::as_str);
                let slots = v.get_i64("slots").ok();
                match spells {
                    Some(spells) => {
                        let mut result = String::new();
                        let k = Ordinal(k).to_string();
                        match lower {
                            Some(lower) => {
                                let lower = Ordinal(lower).to_string();
                                result.push_str(&format!("{}-{}: ", lower, k));
                            }
                            None => {
                                result.push_str(&k);
                            }
                        }

                        match slots {
                            Some(1) => result.push_str("(1 slot) "),
                            Some(slots) => result.push_str(&format!("({} slots )", slots)),
                            None => {}
                        }

                        result.push_str(&spells.join(", "));

                        Some(result)
                    }
                    None => None,
                }
            })
            .collect::<Vec<_>>()
            .to_option()
    }
    fn get_stat(&self, stat: &str) -> Option<String> {
        let num = self.get_i64(stat).ok()?;
        let bonus = (num - 10) / 2;
        if bonus >= 0 {
            Some(format!("{} (+{})", num, bonus))
        } else {
            Some(format!("{} ({})", num, bonus))
        }
    }
}

// fn stat_to_string(stat: i64) -> String {
//     if bonus >= 0 {
//         format!("{} (+{})", num, bonus)
//     } else {
//         format!("{} ({})", num, bonus)
//     }
// }

fn size_to_string(size: &str) -> &str {
    match size {
        "F" => "fine",
        "D" => "diminutive",
        "T" => "tiny",
        "S" => "small",
        "M" => "medium",
        "L" => "large",
        "H" => "huge",
        "G" => "gargantua",
        "C" => "colossal",
        "V" => "varies",
        _ => size,
    }
}

fn alignment_to_string(alignment: &str) -> &str {
    match alignment {
        "L" => "lawful",
        "N" => "neutral",
        "NX" => "neutral (law/chaos axis)",
        "NY" => "neutral (good/evil axis)",
        "C" => "chaotic",
        "G" => "good",
        "E" => "evil",
        "U" => "unaligned",
        "A" => "any alignment",
        _ => alignment,
    }
}
