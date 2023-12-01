use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]

struct Args {
    /// Path to input file
    #[arg(short, long)]
    input: std::path::PathBuf,

    /// Path to output file
    #[arg(short, long)]
    out: String,
}

fn csv(content: String) -> String{
	return content;
}

fn txt(content: String) -> String{
	return content;
}

fn main() {
	let args: Args = Args::parse();
	let content: String = std::fs::read_to_string(&args.input).expect("could not read file");

	let ext: &str = args.input.extension().expect("bad extension").to_str().expect("extension is not string");
	let luau_content: String = match ext {
		"csv" => {
			csv(content)
		},
		"txt" => {
			txt(content)
		},
		_ => {
			String::from("fail")
		}
	};
	// println!("ext: \"{}\"", ext.to_str().expect("strings alright"));
	println!("content: \"{}\"", luau_content);
	println!("out \"{}\"!", args.out);
}