use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use image::io::Reader as ImageReader;

#[derive(Parser)]
#[command(author, version, arg_required_else_help(true))]
#[command(about = "Fish - CLI File converter")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Convert a file
    Convert(Convert),
}

#[derive(Args)]
struct Convert {
    /// Format of output file.
    #[arg(short = 'f', long = "format")]
    format: String,

    /// The input file.
    #[arg(short = 'i', long = "input")]
    input: String,

    /// The output file where save the result.
    #[arg(short = 'o', long = "output")]
    output: Option<String>,
}

#[derive(Args)]
struct NoCommand {
    #[arg(short = 'h', long = "help")]
    help: String,
}

fn convert_fn(args: &Convert) {
    let format = &args.format;
    let input = &args.input;

    let mut path = PathBuf::from(input);
    path.set_extension(format);

    let output_opt = &args.output;
    let mut output = &path.to_str().unwrap().to_string();

    if output_opt.is_some() {
        output = output_opt.as_ref().unwrap();
    }

    println!("[F1sh] Converting file to {}: {}", format, input);

    let source_img = ImageReader::open(input).unwrap().decode().unwrap();
    let result = source_img.save(output);
    if result.is_ok() {
        println!("[F1sh] Task completed.");
    } else {
        let error = result.err().unwrap();
        let msg = error.to_string();
        println!("[F1sh] Failed task with error: \"{}\'", msg);
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Convert(input)) => convert_fn(input),
        None => {}
    }
}
