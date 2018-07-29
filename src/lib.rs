pub mod keybindings_config;

use std::collections::HashMap;
use std::io::Error;

pub trait Config<'c> {

    fn new(filename: &'c str) -> Self;
    fn get(&self, key: &str) -> Option<String>;
    fn get_key_from_value(&self, value: &str) -> Option<String>;
    fn set(&mut self, HashMap<String, String>);
    fn save(&self) -> Result<(), Error>; 

}
