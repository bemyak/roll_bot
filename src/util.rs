use serde_json;
use unqlite::document;

pub fn convert(dvalue: &document::Value) -> serde_json::Value {
    match dvalue {
        document::Value::Null => serde_json::Value::Null,
        document::Value::Int(num) => serde_json::Value::Number(serde_json::Number::from(*num)),
        document::Value::Real(num) => {
            serde_json::Value::Number(serde_json::Number::from_f64(*num).unwrap())
        }
        document::Value::String(string) => serde_json::Value::String(string.to_string()),
        document::Value::Bool(boo) => serde_json::Value::Bool(*boo),
        document::Value::Array(values) => {
            let values = values.iter().map(|value| convert(value)).collect();
            serde_json::Value::Array(values)
        }
        document::Value::Object(map) => {
            let values = map
                .iter()
                .map(|(key, value)| (key.to_string(), convert(value)))
                .collect();
            serde_json::Value::Object(values)
        }
    }
}
