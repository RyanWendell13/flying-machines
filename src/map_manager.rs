extern crate regex;

use std::fs::{File, self};
use std::path::{Path};
use std::io::{Write};
use regex::Regex;

pub fn read_map_from_file(file_name: &str) -> (Vec<Vec<i32>>, [i32;2]){
    let path = format!("maps/{}",file_name);
    let file_contents = fs::read_to_string(path).unwrap();

    let mut map:Vec<Vec<i32>> = Vec::new();
    
    let mut row: Vec<i32> = Vec::new();

    let mut x_size: Option<i32> = None;
    let mut y_size: Option<i32> = None;

    let mut row_index: i32 = 0;
    let mut column_index: i32 = 0;

    let mut spawn: [i32; 2] = [0,0];

    let regular_expression = Regex::new(r"\d+").unwrap();
    for i in regular_expression.captures_iter(&file_contents) {
        if (x_size == None){
            x_size = Some(i[0].parse::<i32>().unwrap());
        }
        else if (y_size == None){
            y_size = Some(i[0].parse::<i32>().unwrap());
        }
        else {
            row.push(i[0].parse::<i32>().unwrap());
            row_index += 1;
            if row_index >= x_size.unwrap() {
                row_index = 0;
                map.push(row);
                column_index+=1;
                row = Vec::new();
            }
            if(i[0].parse::<i32>().unwrap() == 2){
                spawn = [row_index, column_index]
            }
        }
    }

    // for i in 0..map {
        
    // }
    return  (map, spawn);
}

pub fn generate_map_file(file_name: &str, x_size:u32, y_size:u32)->File{
    let path = "maps/".to_owned()+file_name;
    let mut file = if (Path::new(&path).exists()){
            File::options().append(true).open(path).expect("File Cannot Be Opened")
        } 
        else{
            File::create(path).expect("Cannot Create File")
        };


    let mut map:String = String::new();
    for i in 0..y_size {
        for j in 0..x_size {
            if i == 0 || i == y_size-1 || j == 0 {
                map.push_str("1, ");
            }
            else if j == x_size-1 {
                map.push_str("3, ");
            }
            else {
                map.push_str("0, ");
            }
        }
        map.push_str("\n");
        
    }


    let file_contents = format!("size({},{})\n{}", x_size, y_size, map);
    
    file.write_all(file_contents.as_bytes()).unwrap();

    return file;
}

pub fn delete_map_file(file_name:String){
    fs::remove_file(file_name).unwrap();
}