use std::{collections::HashMap, path::PathBuf};
use anyhow::Result;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    pub projector: HashMap<PathBuf, HashMap<String, String>>
}

pub struct Projector {
    config: PathBuf,
    pwd: PathBuf,
    data: Data,
}

fn default_data() -> Data {
    Data {
        projector: HashMap::new(),
    }
}

impl Projector {
    pub fn from_config(config: PathBuf, pwd: PathBuf) -> Self {
        if std::fs::metadata(&config).is_ok() {
            let contents = std::fs::read_to_string(&config);
            let contents = contents.unwrap_or(String::from("{\"projector\": {}"));
            let data = serde_json::from_str(&contents);
            let data = data.unwrap_or(default_data());
            
            return Projector {
                config,
                pwd,
                data,
            };
        }
        
        Projector {
            pwd,
            data: default_data(),
            config,
        }
    }

    pub fn get_value(&self, key: &str) -> Option<&String> {
        let mut curr = Some(self.pwd.as_path());

        let mut out = None;
        while let Some(p) = curr {
            if let Some(dir) = self.data.projector.get(p) {
                if let Some(value) = dir.get(key) {
                    out = Some(value);
                    break;
                }
            }

            curr = p.parent(); 
        }

        out
    }

    pub fn get_value_all(&self) -> HashMap<&String, &String> {
        let mut curr = Some(self.pwd.as_path());
        let mut paths = vec![];
        while let Some(p) = curr {
            paths.push(p);
            curr = p.parent();
        }


        let mut result = HashMap::new();
        for path in paths.into_iter().rev() {
            if let Some(map) = self.data.projector.get(path) {
               result.extend(map.iter()) 
            }
        }

        result
    }

    pub fn set_value(&mut self, key: String, value: String) {
        self.data.projector
            .entry(self.pwd.clone())
            .or_default()
            .insert(key, value);
    }

    pub fn remove_value(&mut self, key: &str) {
        self.data.projector
            .get_mut(&self.pwd)
            .map(|x| {
                x.remove(key);
            });
    }

    pub fn save(&self) -> Result<()> {
        if let Some(p) = self.config.parent() {
            if !std::fs::metadata(&p).is_ok() {
                std::fs::create_dir_all(p)?;
            }
        }
        
        let contents = serde_json::to_string(&self.data)?;
        std::fs::write(&self.config, contents)?;

        Ok(())
    }

}

#[cfg(test)]
mod test {

    use std::{collections::HashMap, path::PathBuf};
    use collection_macros::hashmap;


    use super::{Data, Projector};

    fn get_data() -> HashMap<PathBuf, HashMap<String, String>> {
        hashmap! {
            PathBuf::from("/") => hashmap! {
                "foo".into() => "bar1".into(),
                "baz".into() => "bat".into(),
            },
            PathBuf::from("/foo") => hashmap! {
                "foo".into() => "bar2".into(),
            },
            PathBuf::from("/foo/bar") => hashmap!{
                "foo".into() => "bar3".into(),
            },
        }
    }

    fn get_projector(pwd: PathBuf) -> Projector {
        Projector{
            config: PathBuf::from(""),
            pwd: pwd,
            data: Data {
                projector: get_data(),
            }
        }
    }

    #[test]
    fn test_get_value() {
        let projector = get_projector(PathBuf::from("/foo/bar"));

        assert_eq!(projector.get_value("foo"), Some(&String::from("bar3")));
        assert_eq!(projector.get_value("baz"), Some(&String::from("bat")));
    }


    #[test]
    fn test_set_value() {
        let mut projector = get_projector(PathBuf::from("/foo/bar"));

        projector.set_value(String::from("baz"), String::from("batman"));
        assert_eq!(projector.get_value("baz"), Some(&String::from("batman")));
    }


    #[test]
    fn test_remove_value() {
        let mut projector = get_projector(PathBuf::from("/foo/bar"));

        
        projector.remove_value("foo");
        assert_eq!(projector.get_value("foo"), Some(&String::from("bar2")));

        projector.remove_value("baz");
        assert_eq!(projector.get_value("baz"), Some(&String::from("bat")));
    }
}
