use toml::{Value, map::Map, value::Date, value::Time, value::Offset};

fn table_to_luau(table: &Map<String, Value>) -> String {
	let mut luau_table = "{\n".to_string();
	for (key, value) in table {
		luau_table.push_str(&format!("\t{} = {},\n", key, toml_to_luau(value)));
	}
	luau_table.push_str("}");
	luau_table
}

fn array_to_luau(arr: &[Value]) -> String {
	let mut luau_list = "{\n".to_string();
	for value in arr {
		luau_list.push_str(&format!("\t{},\n", toml_to_luau(value)));
	}
	luau_list.push_str("}");
	luau_list
}

fn toml_to_luau(toml: &Value) -> String {
	match toml {
		Value::Table(table) => table_to_luau(table),
		Value::Array(arr) => array_to_luau(arr),
		Value::String(s) => format!("\"{}\"", s),
		Value::Boolean(b) => format!("{}", b),
		Value::Integer(n) => format!("{}", n),
		Value::Float(f) => format!("{}", f),
		Value::Datetime(dt) => {
			let date: Date = dt.date.expect("bad date");
			let time: Time = dt.time.expect("bad time");
			// let offset: Offset = dt.offset.expect("bad offset");
			format!(
				"DateTime.fromUniversalTime({}, {}, {}, {}, {}, {}, {}/1000000)",
				date.year,
				date.month,
				date.day,
				time.hour,
				time.minute,
				time.second,
				time.nanosecond
			)
		},
		_ => "".to_string(),
	}
}

pub fn translate(content: &str) -> String{
	return toml_to_luau(&toml::from_str(&content).map_err(|e| e.to_string()).expect("couldn't parse toml"))
}