mod json_to_luau;
mod toml_to_luau;
mod yaml_to_luau;

use clap::Parser;
use stylua_lib::{self, Config, OutputVerification, LineEndings, IndentType, QuoteStyle, CallParenType, CollapseSimpleStatement, SortRequiresConfig};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]

struct Args {
    /// Path to input file
    #[arg(short, long)]
    input: std::path::PathBuf,

    /// Path to output file
    #[arg(short, long)]
    out: std::path::PathBuf,
}

fn format_code(code: String) -> String{

	let style_result: Result<String, stylua_lib::Error> = stylua_lib::format_code(
		&code,
		Config {
			column_width: 200,
			line_endings: LineEndings::Windows,
			indent_type: IndentType::Tabs,
			indent_width: 5,
			quote_style: QuoteStyle::AutoPreferDouble,
			no_call_parentheses: false,
			call_parentheses: CallParenType::Always,
			collapse_simple_statement: CollapseSimpleStatement::ConditionalOnly,
			sort_requires: SortRequiresConfig::new(),
		},
		Option::None,
		OutputVerification::Full
	);

	let fmt_content = match style_result {
		Ok(out) => out,
		Err(error) => {
			panic!("Problem styling code: {}, \n{}", error, code)
		}
	};

	return fmt_content;
	// return code
}

fn main() {
	let args: Args = Args::parse();
	let content: String = std::fs::read_to_string(&args.input).expect("could not read file");

	let ext: &str = args.input.extension().expect("bad extension").to_str().expect("extension is not string");
	let luau_content: String = match ext {
		"txt" => {
			format!("\nreturn `{}`", content)
		},
		"json" => {
			format_code(format!("return {}", json_to_luau::translate(&content)))
		},
		"toml" => {
			format_code(format!("return {}", toml_to_luau::translate(&content)))
		},
		"yaml" => {
			format_code(format!("return {}", yaml_to_luau::translate(&content)))
		},
		_ => {
			String::from("return nil")
		}
	};

	std::fs::write(args.out, luau_content).expect("write failed");
}