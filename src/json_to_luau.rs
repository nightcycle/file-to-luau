use serde_json::{Value, Map};

fn object_to_luau(map: &Map<String, Value>) -> String {
    let mut luau_table = "{\n".to_string();
    for (key, value) in map {
        luau_table.push_str(&format!("\t[\"{}\"] = {},\n", key, json_to_luau(value)));
    }
    luau_table.push_str("}");
    luau_table
}

fn array_to_luau(arr: &[Value]) -> String {
    let mut luau_list = "{\n".to_string();
    for value in arr {
        luau_list.push_str(&format!("\t{},\n", json_to_luau(value)));
    }
    luau_list.push_str("}");
    luau_list
}

pub fn json_to_luau(json: &Value) -> String {
	match json {
		Value::Object(map) => object_to_luau(map),
		Value::Array(arr) => array_to_luau(arr),
		Value::String(s) => format!("\"{}\"", s.replace("\n", "\\n").replace("\"", "\\\"").lines().collect::<Vec<&str>>().join("\\n")),
		Value::Bool(b) => {
			format!("{}", b)
		},
		Value::Number(n) => {
			format!("{}", n)
		},
		Value::Null => String::from("nil"),
		// Add cases for other types if needed
		// _ => "".to_string(),
	}
}

pub fn translate(content: &str) -> String{
	return json_to_luau(&serde_json::from_str(content).map_err(|e| e.to_string()).expect("can't parse json"))
}