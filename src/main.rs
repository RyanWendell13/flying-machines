mod machine;
mod map_manager;

use machine::Machine;
use rand::{Rng};
use std::{time::{Duration}, thread};
use std::io::prelude::*;
use std::io;

fn main() {


    //1 = wall, spawn = 2, goal = 3 
    

    let mut machines:Vec<Machine>;
    let pop_num = 300;
    let mut best = vec![0];
    let mut goal_completed = false;
    let mut generation = 0;

    // let mut input = String::new();

    // io::stdin().read_line(&mut input).expect("error");

    let map:Vec<Vec<i32>> = map_manager::read_map_from_file("Map");

    while goal_completed == false {

        machines = new_generation(&best, pop_num, [2,10], 7);
        generation+=1;
        while all_dead(&machines) == false {
            let result = move_machines(&map, &mut machines, &mut goal_completed);
            println!("\x1b[2J");
            println!("{}, Generation: {}",machines.len(), generation);
            print_map(&machines, &map);
            if result != None{
                let res = result.unwrap();
                println!("{:?}", res);
                println!("Path Length: {:?}", res.instructions.len());
                println!("Generation: {}", generation);
                println!("{}", goal_completed);
                break;
            }
            else{
                thread::sleep(Duration::from_millis(25));
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
                if m.position == [j as i32,i as i32]{
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
            if is_machine == false {
                if map[i][j] == 0 {
                    print!(" ");
                }
                else if map[i][j] == 1 {
                    print!("#");
                }
                else if map[i][j] == 2 {
                    print!("X");
                }
                else if map[i][j] == 3 {
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
        if m.dead == false {
            return false;
        }
    }
    return true;
}

fn find_best(machines: &mut Vec<Machine>, x_size: i32) -> Vec<i8> {

    let mut best = 0;
    for i in 0..machines.len(){
        if (x_size - machines[best].position[0]) as f64/((machines[best].instructions.len() as f64)/1.25)  >  (x_size - machines[i].position[0]) as f64/(machines[i].instructions.len() as f64/1.25)  {
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
        if m.dead == false{
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