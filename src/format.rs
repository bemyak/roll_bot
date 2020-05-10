use std::fmt::Write;

use ejdb::bson;

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
