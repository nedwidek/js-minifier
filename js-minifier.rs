use std::fs;
use std::fs::File;
use std::io;
use std::process;
use std::path::PathBuf;
use structopt::StructOpt;

const JS_MINIFIER_URL: &str = "https://javascript-minifier.com/raw";

#[derive(Debug, StructOpt)]
#[structopt(name = "js-minifier", about = "Minify a JavaScript file with javascript-minifier.com.")]
struct Opt {
    /// Source file
    #[structopt(parse(from_os_str))]
    source: PathBuf,

    /// Output file
    #[structopt(parse(from_os_str))]
    output: PathBuf
}

fn main() {
    let opt = Opt::from_args();

    if ! opt.source.exists() || opt.source.is_dir() {
        eprintln!("Source file does not exist or is a directory: {:?}", opt.source);
        process::exit(1);
    }

    do_request(opt.source, opt.output);
}

fn do_request(source: PathBuf, output: PathBuf) {
    let source_str: String = fs::read_to_string(source.into_os_string()).unwrap();

    let params = [("input", source_str)];
    let client = reqwest::blocking::Client::new();
    let response = client.post(JS_MINIFIER_URL)
        .form(&params)
        .send();

    let out = File::create(output.into_os_string());
    io::copy(&mut response.unwrap(), &mut out.unwrap()).expect("Error writing output");
}