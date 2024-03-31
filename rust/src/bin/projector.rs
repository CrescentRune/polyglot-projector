
use clap::Parser;
use rust::opts::ProjectorOpts;

fn main() {
    let opts = ProjectorOpts::parse();
    println!("{:?}", opts);
}
