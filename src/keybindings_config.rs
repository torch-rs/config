extern crate fs2;
extern crate serde;
extern crate serde_yaml;

use self::fs2::FileExt;

use Config;
use std::collections::HashMap;
use std::io;
use std::fs::File;
use std::io::Read;
use std::io::Write;

pub struct KeybindingsConfig {
    filename: String,
    data: HashMap<String, String>,
}

impl Config for KeybindingsConfig {
    
    fn new(filename: String) -> Self {
        match File::open(filename.clone()) {
            Ok(mut file) => {
                file.lock_exclusive().unwrap();
                let mut contents = String::new();
                file.read_to_string(&mut contents).expect("Something went wrong while reading the file");
                file.unlock().unwrap();
                return KeybindingsConfig {
                    filename: filename,
                    data: serde_yaml::from_str(&contents).unwrap(),
                }
            }
            Err(_e) =>  {
                let mut data = HashMap::new();
                data.insert(String::from("show-window"), String::from("Ctrl + SPACE"));
                data.insert(String::from("hide-window"), String::from("ESCAPE"));
                data.insert(String::from("previous-option"), String::from("UP"));
                data.insert(String::from("next-option"), String::from("DOWN"));
                data.insert(String::from("execute-primary-action"), String::from("Enter"));
                data.insert(String::from("execute-secondary-action"), String::from("Alt + ENTER"));
                return KeybindingsConfig {
                    filename: filename,
                    data: data,
                }
            }
        }
    }

    fn get(&self, key: String) -> Option<String> {
        match self.data.get(&key) {
            Some(value) => Some(value.to_owned()),
            None => None,
        }
    }

    fn get_key_from_value(&self, value: String) -> Option<String> {
        for (key, val) in self.data.iter() {
            if val.to_string() == value {
                return Some(key.to_string());
            }
        }
        None
    }

    fn set(&mut self, new_data: HashMap<String, String>) {
        self.data = new_data;
    }
    
    fn save(&self) -> Result<(), io::Error> {
        let serialized = serde_yaml::to_string(&self.data).unwrap();
        let mut file = File::create(self.filename.clone()).unwrap();
        file.lock_exclusive().unwrap();
        if file.write_all(serialized.as_bytes()).is_err() {
            return Err(io::Error::new(io::ErrorKind::Other, "Trouble saving to file"));
        }
        Ok(())
    }

}

#[cfg(test)]
mod tests {

    use Config;
    use keybindings_config::KeybindingsConfig;
    use std::collections::HashMap;
    use std::fs::remove_file;

    #[test]
    fn file_not_created() {
        let mut config = KeybindingsConfig::new(String::from("keybindings_config.yaml"));
        let mut sample_config_data = HashMap::new();
        sample_config_data.insert(String::from("key1"), String::from("value1"));
        sample_config_data.insert(String::from("key2"), String::from("value2"));
        sample_config_data.insert(String::from("key3"), String::from("value3"));
        sample_config_data.insert(String::from("key4"), String::from("value4"));
        sample_config_data.insert(String::from("key5"), String::from("value5"));
        config.set(sample_config_data.clone());
        assert_eq!(config.get(String::from("key1")), Some(String::from("value1")));
        assert_eq!(config.get(String::from("key2")), Some(String::from("value2")));
        assert_eq!(config.get(String::from("key3")), Some(String::from("value3")));
        assert_eq!(config.get(String::from("key4")), Some(String::from("value4")));
        assert_eq!(config.get(String::from("key5")), Some(String::from("value5")));
        assert_eq!(config.get_key_from_value(String::from("value5")), Some(String::from("key5")));
    }

    #[test]
    fn with_file_created() {
        let mut setup_config = KeybindingsConfig::new(String::from("keybindings_config.yaml"));
        let mut sample_config_data = HashMap::new();
        sample_config_data.insert(String::from("key1"), String::from("value1"));
        sample_config_data.insert(String::from("key2"), String::from("value2"));
        sample_config_data.insert(String::from("key3"), String::from("value3"));
        sample_config_data.insert(String::from("key4"), String::from("value4"));
        sample_config_data.insert(String::from("key5"), String::from("value5"));
        setup_config.set(sample_config_data.clone());
        assert!(setup_config.save().is_ok());

        let config = KeybindingsConfig::new(String::from("keybindings_config.yaml"));
        assert_eq!(config.get(String::from("key1")), Some(String::from("value1")));
        assert_eq!(config.get(String::from("key2")), Some(String::from("value2")));
        assert_eq!(config.get(String::from("key3")), Some(String::from("value3")));
        assert_eq!(config.get(String::from("key4")), Some(String::from("value4")));
        assert_eq!(config.get(String::from("key5")), Some(String::from("value5")));
        assert_eq!(config.get_key_from_value(String::from("value5")), Some(String::from("key5")));
        assert!(remove_file("keybindings_config.yaml").is_ok());
    }

}
