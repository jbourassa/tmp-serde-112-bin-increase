#[allow(dead_code)]
#[derive(serde::Deserialize)]
struct Number {
    number: f64,
}

fn main() {
    let json_str = r#"{"number": 42.5}"#;
    let _parsed: Number = serde_json::from_str(json_str).unwrap();
}
