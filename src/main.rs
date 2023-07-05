// https://en.wikipedia.org/wiki/Turing_machine

pub mod tape;
pub mod head;

// 3-state busy beaver
fn busy_beaver(state: char, current_symbol: bool) -> (head::Direction, char, bool) {
    match state {
        'A' => if current_symbol {
            (head::Direction::Left, 'C', true)
        } else {
            (head::Direction::Right, 'B', true)
        },
        'B' => if current_symbol {
            (head::Direction::Right, 'B', true)
        } else {
            (head::Direction::Left, 'A', true)
        },
        'C' => if current_symbol {
            (head::Direction::Right, head::HALT, true)
        } else {
            (head::Direction::Left, 'B', true)
        },
        _ => (head::Direction::Nothing, state, current_symbol)
    }
}

fn main() {
    let mut tp: tape::Tape = tape::Tape::new();
    let mut hd: head::Head = head::Head::new('A', &busy_beaver);

    // To generate the sides at the start, purely visual
    for _ in 0..5 { tp.move_right(); }
    for _ in 0..2 { tp.move_left(); }

    // Main loop
    tp.display();
    while hd.operate(&mut tp) {
        tp.display();
    }
}
