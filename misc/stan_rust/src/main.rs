extern crate simcolor; // https://github.com/vernisaz/simcolor
use simcolor::{Colorized};
use std::error::Error;
//use std::time::Instant;
//use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
    let height_tree = 12;
    let mut level_width = 1;
    let max_width = 1 + height_tree*2;
    for _ in 1..height_tree {
        let level_fill = "*".repeat(level_width);
        let level_ident = " ".repeat((max_width-level_width)/2);
        level_width +=2;
        println!("{level_ident}{}",level_fill.green().on().color_num(70)) // num color from https://hexdocs.pm/color_palette/color_table.html
    }
    Ok(())
}
// not my code