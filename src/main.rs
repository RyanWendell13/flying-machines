use rand::{Rng};
use std::{time::{Duration}, thread};

#[derive(Debug, Clone, PartialEq)]
struct Machine{

    //0 = none, -1 = left, 1 = right
    instructions:Vec<i8>,
    // 0 = right, 1 = down, 2 = left, 3 = up 
    heading: i8,
    position: [i32; 2],
    dead: bool
}
impl Machine{
    fn new(new_instruct: Vec<i8>, new_pos: [i32; 2]) -> Machine{
        Machine { instructions: new_instruct, heading: 0, position: new_pos, dead: false}
    }
}


fn main() {

    
    // i < 0 = 3 i > 3 = 0
    let mut machines:Vec<Machine>;

    //1 = wall, spawn = 2, goal = 3 
    let map1 = [
        [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
        [1,0,0,0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,3],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,3],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,3],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,3],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,3],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,3],
        [1,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3],
        [1,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,3],
        [1,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,1,1,1,0,0,0,0,3],
        [1,0,2,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,1,1,1,1,1,0,0,0,3],
        [1,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,1,1,1,0,0,0,0,3],
        [1,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,3],
        [1,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,3],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,3],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,3],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,3],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,3],
        [1,0,0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,3],
        [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]
    ];

    let map = map1;

    let pop_num = 300;
    let mut best = vec![0];
    let mut goal_completed = false;
    let mut generation = 0;

    while goal_completed == false {

        machines = new_generation(&best, pop_num, [2,10], 7);
        generation+=1;
        let mut i = 0;
        while all_dead(&machines) == false {
            let result = move_machines(map, &mut machines, i, &mut goal_completed);
            println!("\x1b[2J");
            println!("{}, Generation: {}",machines.len(), generation);
            print_map(&machines, map);
            if result != None{
                let res = result.unwrap();
                println!("{:?}", res);
                println!("Path Length: {:?}", res.instructions.len());
                println!("Generation: {}", generation);
                println!("{}", goal_completed);
                break;
            }
            else{
                i+=1;
                thread::sleep(Duration::from_millis(25));
            }
        }
        best = find_best(&mut machines, map[0].len() as i32);

    }
}


fn print_map(machines: &Vec<Machine>, map: [[i32; 61];21]){
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
                    instruction[rng.gen_range(length/2..length)] = rng.gen_range(-1..2);
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

fn move_machines(map: [[i32; 61];21], machines: &mut Vec<Machine>, i: usize, goal_completed: &mut bool) -> Option<Machine> {
    for m in machines {
        if m.instructions.len() <= i {
            m.dead = true
        }
        if m.dead == false {
                m.heading -= m.instructions[i];
                if  m.heading > 3{
                    m.heading = 0;
                }
                else if m.heading < 0 {
                    m.heading = 3;
                } 

                match m.heading {
                    0 => {
                        m.position[0] += 1;
                    }
                    1 => {
                        m.position[1] += 1;
                    }
                    2 => {
                        m.position[0] -= 1;
                    }
                    3 => {
                        m.position[1] -= 1;
                    }
                    _ => {

                    }
                }

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