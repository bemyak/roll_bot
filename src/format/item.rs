use std::fmt::Write;

use ejdb::bson::{self, Bson, Document};

pub fn format_document(doc: bson::Document) -> String {
    let mut res = String::new();
    doc.into_iter().for_each(|(k, v)| match k.as_ref() {
        "_id" => {}
        "name" => write!(&mut res, "**{}**\n\n", v).unwrap(),
        "entries" => {
            let s = match v {
                bson::Bson::Array(arr) => arr
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

fn simple_format(bs: bson::Bson) -> String {
    match bs {
        bson::Bson::FloatingPoint(num) => format!("{}", num),
        bson::Bson::String(s) => s,
        bson::Bson::Array(arr) => arr
            .into_iter()
            .map(|bs| simple_format(bs))
            .collect::<Vec<String>>()
            .join(", "),
        bson::Bson::Document(doc) => doc
            .into_iter()
            .map(|(k, v)| format!("{}: {}", k, simple_format(v)))
            .collect::<Vec<_>>()
            .join(", "),
        bson::Bson::Boolean(b) => match b {
            true => "Yes".to_owned(),
            false => "No".to_owned(),
        },
        bson::Bson::Null => "null".to_owned(),
        bson::Bson::I32(num) => format!("{}", num),
        bson::Bson::I64(num) => format!("{}", num),
        _ => panic!("Unknown type: {:?}", bs.element_type()),
    }
}

pub fn format_spell(_d: &Document) -> Option<String> {
    None
}

pub fn get_source(d: &Document) -> Option<String> {
    let source = get_field(d, "source", Bson::as_str)?;
    let page = get_field(d, "page", Bson::as_i32);
    let srd = get_field(d, "srd", Bson::as_bool);

    let mut result = format!("Source: {}", source);

    if let Some(page) = page {
        result.push_str(&format!(", page {}", page));
    }

    if let Some(srd) = srd {
        if srd {
            result.push_str(". Available in the SRD.");
        }
    } else {
        result.push_str(".");
    }

    Some(result)
}

fn get_field<'a, 'b, T, F>(d: &'a Document, key: &'b str, f: F) -> Option<T>
where
    F: FnOnce(&'a Bson) -> Option<T>,
{
    d.get(key).map(f).flatten()
}
