pub mod keybindings_config;

use std::collections::HashMap;
use std::io::Error;

pub trait Config {

    fn new(filename: String) -> Self;
    fn get(&self, key: String) -> Option<String>;
    fn set(&mut self, HashMap<String, String>);
    fn save(&self) -> Result<(), Error>; 

}
