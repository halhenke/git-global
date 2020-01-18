#[macro_use]
extern crate git_global;
// use git_global::macros::hash_num::*;

fn main() -> () {
    mkHash!(the_thing, "a1", "a2", "a3");
    println!("Macro Hash");

    println!("{}", the_thing.get("a2").unwrap());
    println!("{}", the_thing.invert().get(&1).unwrap());
}
