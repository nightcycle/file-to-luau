use csv::ReaderBuilder;
use serde_json::json;
use std::error::Error;
use crate::json_to_luau;

fn convert_to_json(csv_data: &str, delimiter: u8, key: &Option<String>) -> Result<String, Box<dyn Error>> {
	let mut reader = ReaderBuilder::new().delimiter(delimiter)
		.from_reader(csv_data.as_bytes());

	let headers = reader.headers().expect("bad headers").clone();
	let mut json_array = Vec::new();
	for result in reader.records() {
		let record = result?;
		let json_object = headers.iter().zip(record.iter()).fold(json!({}), |acc, (key, value)| {

			let mut acc_obj = acc.as_object().unwrap().clone();

			if let Ok(v) = value.parse::<i64>() {
				acc_obj.insert(
					key.to_string(),
					json!(v)
				);
			}else if let Ok(v) = value.parse::<f64>() {
				acc_obj.insert(
					key.to_string(),
					json!(v)
				);
			}else if let Ok(v) = value.parse::<bool>() {
				acc_obj.insert(
					key.to_string(),
					json!(v)
				);
			}else {
				acc_obj.insert(
					key.to_string(),
					json!(value)
				);
			}

			acc_obj.into()
		});

		json_array.push(json_object);
	}

	match key {
		Some(k) => {
			// Create a HashMap to store the converted data
			let mut output_data = serde_json::Map::new();

			for item in json_array {
				if let Some(key) = item[k].as_str() {
					// Clone the item and insert it into the output_data with key as the key
					output_data.insert(key.to_string(), item.clone());
				}
			}

			// Serialize the output_data back to JSON
			Ok(serde_json::to_string(&output_data)?)
		},
		None => Ok(serde_json::to_string(&json_array)?)
	}

}

pub fn translate(content: &str, delimiter: u8, key: &Option<String>) -> String{
	return json_to_luau::translate(&convert_to_json(content, delimiter, key).expect("bad sv file"))
}