// extern crate failure;
// extern crate serde;
// extern crate serde_json;
// #[macro_use]
// extern crate serde_derive;
// 
// extern crate regex;
// #[macro_use]
// extern crate lazy_static;
// 
// use curl::easy::Easy;
// use regex::Regex;
// use std::collections::BTreeMap;
// use std::fs::File;
// use std::io::Write;

use std::{env, process};
use getopts::Options;

#[derive(Debug)]
struct Args {
  input: Vec<String>,
  output: Option<String>,
  // ...
}

fn print_usage(program: &str, opts: &Options) {
  let brief = format!("Usage: {} FILE [options]", program);
  print!("{}", opts.usage(&brief));
  process::exit(0);
}

fn parse_args() -> Args {
  let args: Vec<String> = env::args().collect();
  let program = args[0].clone();

  let mut opts = Options::new();
  opts.optopt("o", "", "set output file name", "NAME");
  opts.optflag("h", "help", "print this help menu");
  // ...

  let matches = opts.parse(&args[1..])
    .unwrap_or_else(|f| panic!(f.to_string()));

  if matches.opt_present("h") {
    print_usage(&program, &opts);
  }

  if matches.free.is_empty() {
    print_usage(&program, &opts);
  }

  Args {
    input: matches.free.clone(),
    output: matches.opt_str("o"),
    // ...
  }
}

fn main() {
  let args = parse_args();
  println!("{:?}", args);
}
//lazy_static! {
//    static ref CREATED_FILE_PATH: String = {
//        let working_directory = env!("CARGO_MANIFEST_DIR");
//        let file_path = "src/entities.rs";
//        format!("{}/{}", working_directory, file_path)
//    };
//}
//const REQUEST_URL: &'static str = "https://html.spec.whatwg.org/entities.json";
//const BOILERPLATE: &'static str = r#"pub static MINIMAL_ENTITIES: [(char, &'static str); 5] = [
//    ('"', "&quot;"),
//    ('&', "&amp;"),
//    ('\'', "&#x27;"),
//    ('<', "&lt;"),
//    ('>', "&gt;")
//];
//pub static NAMED_ENTITIES: &'static[(&'static str, &'static str)] = &[
//"#;
//#[derive(Deserialize, Debug)]
//struct EntityData {
//    codepoints: Vec<u32>,
//    characters: String,
//}
//
//#[derive(Deserialize, Debug)]
//struct HtmlEntity {
//    nodes: BTreeMap<String, EntityData>,
//}
//
//fn request() -> Result<Vec<u8>, failure::Error> {
//    let mut dst = Vec::new();
//    let mut easy = Easy::new();
//
//    easy.url(REQUEST_URL)?;
//
//    {
//        let mut transfer = easy.transfer();
//        transfer
//            .write_function(|data| {
//                dst.extend_from_slice(data);
//                Ok(data.len())
//            })
//            .unwrap();
//        transfer.perform()?;
//    }
//
//    Ok(dst)
//}
//
//fn parse_json(data: &str) -> Result<String, failure::Error> {
//    let mut result_str = String::new();
//    let json: HtmlEntity = serde_json::from_str(data)?;
//    let re = Regex::new(r"&|;")?;
//
//    for (entity_name, data) in json.nodes {
//        if entity_name.ends_with(";") {
//            let mut characters = String::new();
//            for c in data.characters.chars() {
//                characters.push_str(&format!("{}", c.escape_unicode()));
//            }
//            result_str.push_str(&format!(
//                "(\"{}\", \"{}\"),\n",
//                re.replace_all(&entity_name, ""),
//                characters
//            ));
//        }
//    }
//
//    Ok(result_str)
//}

//    let json = request().unwrap();
//    let json = format!("{{\"nodes\": {} }}", std::str::from_utf8(&json).unwrap());
//
//    let parsed_str = parse_json(&json).unwrap();
//
//    let result = format!("{}{}];", BOILERPLATE, parsed_str);
//
//    let mut f = File::create(&*CREATED_FILE_PATH).unwrap();
//    f.write_all(result.as_bytes()).unwrap();