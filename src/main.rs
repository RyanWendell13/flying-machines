mod map_manager;
mod machine;
mod settings;

use machine::Machine;
use settings::Settings;
use rand::{Rng};
use std::{time::{Duration}, thread};
use std::io;
use std::fs;

fn main() {
    println!("\x1b[2J");
    let mut map_selected: bool = false;
    let mut map:Vec<Vec<i32>> = Vec::new();
    let mut spawn: [i32; 2] = [0,0];
    let mut settings = Settings::new_test(String::new(), 300, 7, 50);
    main_menu(&mut map, &mut map_selected, &mut spawn, &mut settings);
}

fn main_menu(map: &mut Vec<Vec<i32>>, map_selected: &mut bool, spawn: &mut [i32;2], settings: &mut Settings){
    let mut input = String::new();

    println!("MAIN MENU");
    println!("1. Run Simulation");
    println!("2. Choose/Create Maps");
    println!("3. Settings");
    println!("4. Help");
    println!("Enter Option: ");
    io::stdin().read_line(&mut input).expect("error");
    println!("\x1b[2J");
    input = input.trim().to_string();
    if(input == "1"){
        if(*map_selected == false){
            main_menu(map,map_selected, spawn, settings);
        }
        else {
            run_simulation(map, map_selected, spawn, settings);
        }
    }

    if(input == "2"){
        mapping_menu(map, map_selected, spawn, settings);

    }
    if(input == "3"){

    }
    if(input == "4"){
        println!("INSTRUCTIONS");
        println!("To run a simulation a map must be selected. Once returned to the main menu select option 2");
        println!("If you want to create a new custom map select 3. And follow the instructions. If you want to use a preexisting map select option 1");
        println!("Enter the name of the map you want to use. Once a map is selected you can run the simulation by entering 1 on the main menu.");
        println!("Enter Any Character To Return: ");
        io::stdin().read_line(&mut input).expect("error");
        println!("\x1b[2J");
        main();
    }
    
}

fn mapping_menu(map: &mut Vec<Vec<i32>>, map_selected: &mut bool, spawn: &mut [i32;2], settings: &mut Settings){
    let mut input = String::new();

    println!("MAPPING MENU");
    println!("1. Select Map");
    println!("2. Create Map");
    println!("3. Delete Map");
    println!("4. How To Create And Edit Maps");
    println!("5. Return To Main Menu");
    println!("Enter Option: ");
    io::stdin().read_line(&mut input).expect("error");
    println!("\x1b[2J");
    input = input.trim().to_string();
    

    if (input == "1") {
        input = String::new();

        println!("SELECT MAP");
        println!("Names");
        for file in fs::read_dir("./maps").unwrap() {
            println!("- {}", file.unwrap().file_name().into_string().unwrap());
        }

        println!("Enter The Map's Name: ");
        io::stdin().read_line(&mut input).expect("error");
        println!("\x1b[2J");
        let result = map_manager::read_map_from_file(input.trim());
        *map = result.0;
        *spawn = result.1;
        *map_selected = true;
        main_menu(map, map_selected,spawn, settings)
    }
    if (input == "2") {
        println!("CREATE MAP");
        let mut name = String::new();
        let mut length = String::new();
        let mut height = String::new();
        println!("Enter The Map's Name: ");
        io::stdin().read_line(&mut name).expect("error");

        input = String::new();
        println!("Enter The Length: ");
        io::stdin().read_line(&mut length).expect("error");

        input = String::new();
        println!("Enter The Height: ");
        io::stdin().read_line(&mut height).expect("error");

        map_manager::generate_map_file(name.trim(), length.trim().parse().unwrap(), height.trim().parse().unwrap());
        println!("\x1b[2J");
        mapping_menu(map, map_selected, spawn, settings)

    }
    if(input == "3"){
        input = String::new();
        println!("DELETE MAP");
        println!("Names");
        for file in fs::read_dir("./maps").unwrap() {
            println!("- {}", file.unwrap().file_name().into_string().unwrap());
        }
        println!("Enter The Map's Name: ");
        io::stdin().read_line(&mut input).expect("error");
        println!("\x1b[2J");
        map_manager::delete_map_file(input.trim().to_string());
        main_menu(map, map_selected,spawn,settings)
    }
    if (input == "4") {
        input = String::new();
        println!("INSTRUCTIONS");
        println!("To create a map simple select option 2 in the map menu.");
        println!("Follow to create map file. To edit the map after it is created.");
        println!("Find the file in the maps folder, and open it in a text editor. ");
        println!("0 = empty space, 1 = wall, 2 = start position, 3 = goal");
        println!("Simply swap out the number to modify the map.");
        println!("For best result, keep the preset walls made of 1's and 3's.");
        println!("Add one 2 or start position to the map, and place walls internally as you see fit.");
        print!("Enter Any Character To Return: ");
        io::stdin().read_line(&mut input).expect("error");
        println!("\x1b[2J");
        mapping_menu(map, map_selected, spawn, settings);
    }

    if (input == "5"){
        main_menu(map, map_selected,spawn, settings);
    }
}




fn run_simulation(map: &mut Vec<Vec<i32>>, map_selected: &mut bool, spawn: &mut [i32;2], settings: &mut Settings){
    let mut input = String::new();
    let mut machines:Vec<Machine>;
    let mut best = vec![0];
    let mut goal_completed = false;
    let mut generation = 0;

    while (goal_completed == false) {

        machines = new_generation(&best, settings.pop_num, *spawn, settings.mutations);
        generation+=1;
        while (all_dead(&machines) == false) {
            let result = move_machines(&map, &mut machines, &mut goal_completed);
            println!("\x1b[2J");
            println!("{}, Generation: {}",machines.len(), generation);
            print_map(&machines, &map);
            if (result != None){
                let res = result.unwrap();
                println!("{:?}", res);
                println!("Path Length: {:?}", res.instructions.len());
                println!("Generation: {}", generation);
                println!("Enter Any Character To Return: ");
                io::stdin().read_line(&mut input).expect("error");
                println!("\x1b[2J");
                mapping_menu(map, map_selected, spawn, settings);
            }
            else{
                thread::sleep(Duration::from_millis(settings.delay));
            }
        }
        best = find_best(&mut machines, map[0].len() as i32);    
    }
}

fn print_map(machines: &Vec<Machine>, map: &Vec<Vec<i32>>){
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let mut is_machine = false;
            for m in machines {
                if (m.position == [j as i32,i as i32]){
                    is_machine = true;
                    match m.heading {
                        0 => {
                            print!(">");
                        }
                        1 => {
                            print!("V");
                        }
                        2 => {
                            print!("<");
                        }
                        3 => {
                            print!("^");
                        }
                        _ =>{}
                    }
                    break;

                }
            }
            if (is_machine == false) {
                if (map[i][j] == 0) {
                    print!(" ");
                }
                else if (map[i][j] == 1) {
                    print!("#");
                }
                else if (map[i][j] == 2) {
                    print!("X");
                }
                else if (map[i][j] == 3) {
                    print!("$");
                }
            }
            print!(" ");
        }
        println!();
    }
}

fn all_dead(machines: &Vec<Machine>) -> bool {
    for m in machines {
        if (m.dead == false) {
            return false;
        }
    }
    return true;
}

fn find_best(machines: &mut Vec<Machine>, x_size: i32) -> Vec<i8> {

    let mut best = 0;
    for i in 0..machines.len(){
        if ((x_size - machines[best].position[0]) as f64/((machines[best].instructions.len() as f64)/1.25)  >  (x_size - machines[i].position[0]) as f64/(machines[i].instructions.len() as f64/1.25))  {
            best = i
        }
    }
    return machines[best].instructions.clone()
}

fn new_generation(best:& Vec<i8>, pop_num: i32, pos:[i32;2], mutations: i32) -> Vec<Machine> {
    let mut new_machines:Vec<Machine> = Vec::new();
    let  mut rng = rand::thread_rng();
    new_machines.push(Machine::new(best.clone(), pos));
    for _ in 0..pop_num-1 {
        let mut instruction = best.clone();
        for _ in 0..mutations {
            let r = rng.gen_range(0..3);
            match r {
                0 =>{
                    let length = instruction.len();
                    instruction[rng.gen_range(0..length)] = rng.gen_range(-1..2);
                }
                1 =>{
                    instruction.push(rng.gen_range(-1..2))
                }
                _ =>{}
            }
        }

        new_machines.push(Machine::new(instruction, pos));
    }

    return new_machines
}

fn move_machines(map: &Vec<Vec<i32>>, machines: &mut Vec<Machine>, goal_completed: &mut bool) -> Option<Machine> {
    for m in machines {
        if (m.dead == false){
            m.move_machine();
            match map[m.position[1] as usize][m.position[0] as usize] {
                1 =>{
                    m.dead = true;
                }
                3 =>{
                    *goal_completed = true;
                    return Some(m.clone());
                }
                _ =>{

                }
            }
        }
    }
    return None;
}