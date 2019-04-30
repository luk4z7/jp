extern crate clap;
extern crate ansi_term;
extern crate json;

use clap::{Arg, App};
use ansi_term::{Colour};
use std::fmt;
use std::string::ToString;

#[derive(Debug)]
pub struct PrettyFormatter {
    string: String,
    indent: i32
}

impl PrettyFormatter {
   pub fn from_str(s: &str) -> PrettyFormatter {
        PrettyFormatter {
            string: s.to_owned(),
            indent: 4
        }
    }

    pub fn from_string(s: &String) -> PrettyFormatter {
        PrettyFormatter {
            string: s.clone(),
            indent: 4
        }
    }

    pub fn indent(&mut self, indent: i32) -> PrettyFormatter {
        PrettyFormatter {
            string: self.string.clone(),
            indent: indent,
        }
    }

    pub fn pretty(&self) -> String {
        let mut result = String::new();

        let mut in_string = false;
        let mut indent = 0;
        let mut need_indent = false;

        for ch in self.string.chars() {
            match ch {
                '{' => {
                    if need_indent {
                        for _ in 0..indent {
                            result.push(' ');
                        }
                        need_indent = false;
                    }

                    result.push('{');
                    if !in_string {
                        indent += self.indent;
                        result.push('\n');
                        need_indent = true;
                    }
                },
                '}' => {
                    if !in_string {
                        result.push('\n');
                        indent -= self.indent;
                        for _ in 0..indent {
                            result.push(' ');
                        }
                    }
                    result.push('}');
                },
                '"' => {
                    if need_indent {
                        for _ in 0..indent {
                            result.push(' ');
                        }
                    }
                    result.push('"');
                    in_string = !in_string;
                    need_indent = false;
                },
                ',' => {
                    if need_indent {
                        for _ in 0..indent {
                            result.push(' ');
                        }
                        need_indent = false;
                    }
                    result.push(',');
                    if !in_string {
                        result.push('\n');
                        need_indent = true;
                    }
                },
                ch @ ' ' | ch @ '\t' => {
                    if in_string {
                        result.push(ch);
                    }else{
                        if need_indent {
                            continue;
                        }else{
                            result.push(ch);
                        }
                    }
                },
                '\n' => {
                    if in_string {
                        result.push('\n');
                    }else{
                        need_indent = true;
                        continue;
                    }
                }
                c => {
                    if need_indent {
                        for _ in 0..indent {
                            result.push(' ');
                        }
                    }
                    need_indent = false;
                    result.push(c);
                },
            }
        }

        result
    }
}

impl fmt::Display for PrettyFormatter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.pretty())
    }
}

fn main() {
    let matches = App::new("j is json")
        .version("0.0.1")
        .author("Lucas Alves <luk4z_7@hotmail.com>")
        .about("Parse json to pretty json")
        .arg(Arg::with_name("json")
                .short("j")
                .long("json")
                .takes_value(true)
                .help("pretty json"))
        .arg(Arg::with_name("key")
                .short("k")
                .long("key")
                .takes_value(true)
                .help("key of your structure, ex: -k data.key"))
        .get_matches();

    let j = matches.value_of("json");
    let k = matches.value_of("key").unwrap_or("key");

    match j {
        None => println!("{}", Colour::Red.paint("Nothing to pretty")),
        Some(s) => {

            let parsed = json::parse(&s.to_string()).unwrap();;
            let mut data = parsed.dump();
            
            if k != "key" {
                data = parsed[k].dump();
            }
            
            let formatter = PrettyFormatter::from_str(&data.to_string());
            let result = formatter.pretty();
            println!("{}", Colour::Green.paint(result));
       }
    }
}
