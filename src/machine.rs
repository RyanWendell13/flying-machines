#[derive(Debug, Clone, PartialEq)]
pub struct Machine{

    //0 = none, -1 = left, 1 = right
    pub instructions:Vec<i8>,
    // 0 = right, 1 = down, 2 = left, 3 = up 
    pub heading: i8,
    pub position: [i32; 2],
    pub dead: bool,
    pub index: usize
}
impl Machine{
    pub fn new(new_instruct: Vec<i8>, new_pos: [i32; 2]) -> Machine{
        Machine { instructions: new_instruct, heading: 0, position: new_pos, dead: false, index: 0}
    }
    pub fn move_machine(&mut self){
        if !(self.instructions.len() > self.index) {
            self.dead = true
        }
        if self.dead == false {
            self.heading -= self.instructions[self.index];
            if  self.heading > 3{
                self.heading = 0;
            }
            else if self.heading < 0 {
                self.heading = 3;
            } 
            match self.heading {
                0 => {
                    self.position[0] += 1;
                }
                1 => {
                    self.position[1] += 1;
                }
                2 => {
                    self.position[0] -= 1;
                }
                3 => {
                    self.position[1] -= 1;
                }
                _ => {

                }
            }
        }
        self.index+=1;
    }
}
