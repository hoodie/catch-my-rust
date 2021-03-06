extern crate toml;

use std::fs::File;
use std::io::Read;
use std::path::{Path};
use std::error::Error;

pub struct Config
{
    pub sleep_time: i64,
    pub stations: Vec<String>
}

pub fn read_config() -> Config
{
    let config_path = Path::new("./config.toml");
    let mut file = match File::open(&config_path){
        Err(why) => panic!("[ ERROR ] opening {} because: \"{}\"", config_path.display(), Error::description(&why)),
        Ok(file) => file };

    let mut config_string = String::new();
    match file.read_to_string(&mut config_string){
        Err(why) => panic!("[ ERROR ] could not open file, because: {}", why),
        Ok(_) => () }

    let config = match toml::Parser::new(&config_string).parse() {
        Some(thing) => toml::Value::Table(thing),
        None => panic!("could not parse config file") };

    let mut stations_vec = Vec::new();
    for station in config.lookup("stations").unwrap().as_slice().unwrap() {
        stations_vec.push((station.as_str().unwrap().to_string())) }

    let sleep_time = config.lookup("sleep_time").unwrap().as_integer();
    Config{
        sleep_time: sleep_time.unwrap_or(30) * 1000,
        stations: stations_vec
    }
}



