use csv::ReaderBuilder;
use serde_json::json;
use std::error::Error;
use crate::json_to_luau;

fn convert_to_json(csv_data: &str, delimiter: u8) -> Result<String, Box<dyn Error>> {
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

	Ok(serde_json::to_string(&json_array)?)
}

pub fn translate(content: &str, delimiter: u8) -> String{
	return json_to_luau::translate(&convert_to_json(content, delimiter).expect("bad sv file"))
}