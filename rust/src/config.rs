use std::path::PathBuf;

use anyhow::{Result, anyhow};


pub enum Operation {
    Print(Option<String>),
    Add(String, String),
    Remove(String),
}

impl TryFrom<Vec<String>> for Operation {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self> {
        let mut value = value;
        if value.len() == 0 {
            return Ok(Operation::Print(None));
        }

        let term = value.get(0).expect("expect to exist");
        if term == "add" {
            if value.len() != 3 {
                let err = anyhow!("operation add expects 2 arguments but got{}", value.len() - 1);
                return Err(err);
            }

            let mut drain = value.drain(1..2);
            return Ok(
                Operation::Add(
                    drain.next().expect("to exist"),
                    drain.next().expect("to exist"),
                ));
            
        }

        if term == "delete" {
            if value.len() != 2 {
                let err = anyhow!("operation delete expects 2 arguments but got{}", value.len() - 1);
                return Err(err);
            }

            let arg = value.pop().expect("to exist");
            return Ok(Operation::Remove(arg));
        }

        if value.len() > 1 {
            let err = anyhow!("operation delete expects 0 arguments but got{}", value.len() - 1);
            return Err(err);
        }
        let arg = value.pop().expect("to exist");
       
        Ok(Operation::Print(Some(arg)))
    }
}

fn get_config(config: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(v) = config {
        return Ok(v);
    }
    
    let loc = std::env::var("HOME").context("unable to get home dir from ENV var")?;
    let mut loc = PathBuf::from(loc);
    loc.push("projector");
    loc.push("projector.json");

    return Ok(loc);
}
