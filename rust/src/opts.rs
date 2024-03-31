// src/opts.rs
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap()]
pub struct ProjectorOpts {
    
    #[clap(short = 'p', long = "pwd")]
    pub pwd: Option<PathBuf>,

    #[clap(short = 'c', long = "config")]
    pub config: Option<PathBuf>,

    #[clap(default_value = "")]
    pub operation: Vec<String>,
}
