use serde_yaml::{Value, Mapping};
use chrono::{NaiveDateTime, DateTime, FixedOffset, Datelike, Timelike};

const FORMATS: &[&str; 7] = &[
	"%Y-%m-%d %H:%M:%S%.f%:z", // Complete format with timezone and fractional seconds
	"%Y-%m-%d %H:%M:%S%:z",     // Format with timezone, without fractional seconds
	"%Y-%m-%d %H:%M%:z",        // Format with timezone, without seconds
	"%Y-%m-%d %H:%M:%S%.f",     // Format without timezone, with fractional seconds
	"%Y-%m-%d %H:%M:%S",        // Format without timezone and fractional seconds
	"%Y-%m-%d %H:%M",           // Format without timezone and seconds
	"%Y-%m-%d",                 // Date only
];


fn map_to_luau(map: &Mapping) -> String {
	let mut luau_table = "{\n".to_string();
	for (key, value) in map {
		let key_str = if let Value::String(s) = key {
			s.clone()
		} else {
			panic!("bad key");
		};
		luau_table.push_str(&format!("\t{} = {},\n", key_str, yaml_to_luau(value)));
	}
	luau_table.push_str("}");
	luau_table
}

fn sequence_to_luau(seq: &[Value]) -> String {
    let mut luau_list = "{\n".to_string();
    for value in seq {
        luau_list.push_str(&format!("\t{},\n", yaml_to_luau(value)));
    }
    luau_list.push_str("}");
    luau_list
}

fn format_datetime_to_luau(datetime: &NaiveDateTime) -> String {
	let date = datetime.date();
	let time = datetime.time();

	format!(
		"DateTime.fromUniversalTime({}, {}, {}, {}, {}, {}, {}/1000000)",
		date.year(),
		date.month(),
		date.day(),
		time.hour(),
		time.minute(),
		time.second(),
		time.nanosecond()
	)
}

fn yaml_to_luau(yaml: &Value) -> String {
	match yaml {
		Value::Mapping(map) => map_to_luau(map),
		Value::Sequence(seq) => sequence_to_luau(seq),
		Value::String(s) => {

			for &format in FORMATS {
				if let Ok(dt) = DateTime::parse_from_str(s, format) {
					return format_datetime_to_luau(&dt.naive_local());
				}
				if let Ok(ndt) = NaiveDateTime::parse_from_str(s, format) {
					return format_datetime_to_luau(&ndt);
				}
			}

			format!("\"{}\"", s)
		},
		Value::Bool(b) => format!("{}", b),
		Value::Number(n) if n.is_i64() => format!("{}", n.as_i64().unwrap()),
		Value::Number(n) if n.is_f64() => format!("{}", n.as_f64().unwrap()),
		Value::Null => "nil".to_string(),
		// Handle other specific YAML types as needed
		_ => "".to_string(),
	}
}

pub fn translate(content: &str) -> String{
	return yaml_to_luau(&serde_yaml::from_str(&content).map_err(|e| e.to_string()).expect("couldn't parse yaml"))
}