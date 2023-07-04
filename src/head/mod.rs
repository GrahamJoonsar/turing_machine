use crate::tape;

pub const HALT: char = 255 as char;

pub enum Direction {
    Left,
    Right,
    Nothing
}
#[allow(dead_code)]
pub struct Head <'a> {
    state: char,
    decider: &'a dyn Fn(char, bool) -> (Direction, char, bool)
}

#[allow(dead_code)]
impl <'a> Head <'a> {
    pub fn new(intial_state: char, decider: &'a dyn Fn(char, bool) -> (Direction, char, bool)) -> Head<'a> {
        Head {
            state: intial_state,
            decider
        }
    }

    pub fn operate(&mut self, tp: &mut tape::Tape) {
        let (dir, new_state, new_symbol) = (self.decider)(self.state, tp.get());
        self.state = new_state;
        tp.set(new_symbol);
        match dir {
            Direction::Left  => tp.move_left(),
            Direction::Right => tp.move_right(),
            Direction::Nothing => (),
        }
    }
}