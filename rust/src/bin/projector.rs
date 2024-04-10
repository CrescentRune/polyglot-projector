
use clap::Parser;
use rust::opts::ProjectorOpts;

use anyhow::Result;

fn main() -> Result<()> {
    let opts: Config = ProjectorOpts::parse().try_into()?;
    println!("{:?}", opts);

    Ok(())
}
