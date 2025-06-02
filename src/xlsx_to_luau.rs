use calamine::{open_workbook, Reader, Xlsx};
use std::error::Error;
use crate::csv_to_luau;

pub fn xlsx_to_csv(file_path: &str, specified_page: &Option<String>) -> Result<String, Box<dyn Error>> {
	let mut workbook: Xlsx<_> = open_workbook(file_path.to_string())?;

	let first_sheet: String = match workbook.sheet_names().into_iter().next() {
		Some(sheet) => sheet,
		None => panic!("xlsx book is empty: {}", file_path)
	};

	let sheet_name: &String = match specified_page {
	    Some(page) => page,
	    None => &first_sheet
	};
	let range = workbook
	    .worksheet_range(sheet_name)?;

	let mut csv_content = Vec::new();

	for row in range.rows() {
		let csv_row = row.iter().map(|cell| {
			let cell_str = cell.to_string();
			// Check if the cell content needs to be enclosed in double quotes
			if cell_str.contains(',') || cell_str.contains('\n') || cell_str.contains('"') {
				// Escape double quotes and enclose in double quotes
				format!("\"{}\"", cell_str.replace("\"", "\"\""))
			} else {
				cell_str
			}
		}).collect::<Vec<_>>().join(",");

		csv_content.push(csv_row);
	}

	return Ok(csv_content.join("\n"))
}

pub fn translate(file_path: &str, specified_page: &Option<String>, key: &Option<String>) -> String {
    let res = xlsx_to_csv(file_path, specified_page);
    let csv_content: String = res.expect("bad xlsx");
    csv_to_luau::translate(&csv_content, b',', key)
}