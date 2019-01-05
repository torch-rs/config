extern crate fs2;
extern crate serde;
extern crate serde_yaml;

use self::fs2::FileExt;
use Config;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, ErrorKind, Read, Write};

pub struct KeybindingsConfig<'c> {
    filename: &'c str,
    data: HashMap<String, String>,
}

impl<'c> Config<'c> for KeybindingsConfig<'c> {
    
    fn new(filename: &'c str) -> Result<Self, Error> {
        match File::open(filename) {
            Ok(mut file) => {
                file.lock_exclusive()
                    .map_err(|_e| Error::new(ErrorKind::Other, "Trouble locking file"))?;
                let mut contents = String::new();
                file.read_to_string(&mut contents).expect("Something went wrong while reading the file");
                file.unlock()
                    .map_err(|_e| Error::new(ErrorKind::Other, "Trouble locking file"))?;
                Ok(KeybindingsConfig {
                    filename: filename,
                    data: serde_yaml::from_str(&contents).unwrap(),
                })
            }
            Err(_e) =>  {
                let mut data = HashMap::new();
                data.insert(String::from("close-window"), String::from("ESCAPE"));
                data.insert(String::from("previous-option"), String::from("UP"));
                data.insert(String::from("next-option"), String::from("DOWN"));
                data.insert(String::from("execute-primary-action"), String::from("RETURN"));
                data.insert(String::from("execute-secondary-action"), String::from("Alt + RETURN"));
                Ok(KeybindingsConfig {
                    filename: filename,
                    data: data,
                })
            }
        }
    }

    fn get(&self, key: &str) -> Option<String> {
        match self.data.get(key) {
            Some(value) => Some(value.to_owned()),
            None => None,
        }
    }

    fn get_key_from_value(&self, value: &str) -> Option<String> {
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
    
    fn save(&self) -> Result<(), Error> {
        let serialized = serde_yaml::to_string(&self.data)
            .map_err(|_e| Error::new(ErrorKind::Other, "Trouble serializing"))?;
        let mut file = File::create(self.filename.clone())?;
        file.lock_exclusive()
            .map_err(|_e| Error::new(ErrorKind::Other, "Trouble locking file"))?;
        file.write_all(serialized.as_bytes())
            .map_err(|_e| Error::new(ErrorKind::Other, "Trouble saving to file"))?;
        file.unlock()
            .map_err(|_e| Error::new(ErrorKind::Other, "Trouble unlocking file"))?;
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
        let wrapped_config = KeybindingsConfig::new("keybindings_config.yaml");
        assert!(wrapped_config.is_ok());
        let mut config = wrapped_config.unwrap();
        let mut sample_config_data = HashMap::new();
        sample_config_data.insert(String::from("key1"), String::from("value1"));
        sample_config_data.insert(String::from("key2"), String::from("value2"));
        sample_config_data.insert(String::from("key3"), String::from("value3"));
        sample_config_data.insert(String::from("key4"), String::from("value4"));
        sample_config_data.insert(String::from("key5"), String::from("value5"));
        config.set(sample_config_data.clone());
        assert_eq!(config.get("key1"), Some(String::from("value1")));
        assert_eq!(config.get("key2"), Some(String::from("value2")));
        assert_eq!(config.get("key3"), Some(String::from("value3")));
        assert_eq!(config.get("key4"), Some(String::from("value4")));
        assert_eq!(config.get("key5"), Some(String::from("value5")));
        assert_eq!(config.get_key_from_value("value5"), Some(String::from("key5")));
    }

    #[test]
    fn with_file_created() {
        let wrapped_setup_config = KeybindingsConfig::new("keybindings_config.yaml");
        assert!(wrapped_setup_config.is_ok());
        let mut setup_config = wrapped_setup_config.unwrap();
        let mut sample_config_data = HashMap::new();
        sample_config_data.insert(String::from("key1"), String::from("value1"));
        sample_config_data.insert(String::from("key2"), String::from("value2"));
        sample_config_data.insert(String::from("key3"), String::from("value3"));
        sample_config_data.insert(String::from("key4"), String::from("value4"));
        sample_config_data.insert(String::from("key5"), String::from("value5"));
        setup_config.set(sample_config_data.clone());
        assert!(setup_config.save().is_ok());

        let wrapped_config = KeybindingsConfig::new("keybindings_config.yaml");
        assert!(wrapped_config.is_ok());
        let config = wrapped_config.unwrap();
        assert_eq!(config.get("key1"), Some(String::from("value1")));
        assert_eq!(config.get("key2"), Some(String::from("value2")));
        assert_eq!(config.get("key3"), Some(String::from("value3")));
        assert_eq!(config.get("key4"), Some(String::from("value4")));
        assert_eq!(config.get("key5"), Some(String::from("value5")));
        assert_eq!(config.get_key_from_value("value5"), Some(String::from("key5")));
        assert!(remove_file("keybindings_config.yaml").is_ok());
    }

    #[test]
    fn with_file_not_created_and_valid_bindings() {
        let wrapped_config = KeybindingsConfig::new("with_file_not_created_and_valid_bindings.yaml");
        assert!(wrapped_config.is_ok());
        let config = wrapped_config.unwrap();
        assert_eq!(config.get_key_from_value("Alt + RETURN"),
                   Some(String::from("execute-secondary-action")));
    }

}
