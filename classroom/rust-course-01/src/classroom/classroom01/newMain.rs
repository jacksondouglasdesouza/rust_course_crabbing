use ferris_says::say;
use std::io::{stdout, bufwriter};

fn main (){
    let out = "Hello fellow Rustaceans!";
    let width = out.len();
    let mut writer = bufwriter::new(stdout());
    say(out, width, &mut writer).unwrap();
}