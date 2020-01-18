extern crate colored;
use self::colored::*;
extern crate itertools;
use self::itertools::Itertools;

pub fn print_str_pat<'a>(string: &'a str, pat: Option<&str>) -> String {
    // fn print_str_pat<'a>(string: &'a str, pat: Option<&str>) -> &'a str {
    if let Some(pat) = pat {
        // let split: Vec<ColoredString> = string
        let split: String = string
            // let split: Vec<&str> = string
            .split(pat)
            .map(|x| x.blue())
            .intersperse(pat.green())
            .join("")
            .to_string();
        // println!("{}", split);
        // .for_each(|x| print!("{}", x))
        // .collect();
        return split;
    } else {
        // io::stdout().write(string.as_ptr());
        // println!("{}", string.blue());
        return string.blue().to_string();
    }
}
