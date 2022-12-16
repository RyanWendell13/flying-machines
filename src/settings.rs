extern crate regex;

use std::fs::{File, self};
use std::path::{Path};
use std::io::{Write};
use regex::Regex;


pub struct Settings{
    //hidden in menu
    pub map_name: String,
    pub pop_num: i32,
    pub mutations: i32,
    pub delay: u64
}
impl Settings{
    pub fn new_test(new_map_name:String, new_pop_num:i32, new_mutations:i32, new_delay:u64) -> Settings{
        return Settings{map_name: new_map_name, pop_num:new_pop_num, mutations:new_mutations, delay:new_delay};
    }
    pub fn new(file_name: &str) -> (){
        
    }
}

