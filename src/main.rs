use std::env;

struct State<'a> {
    stream: &'a str, // Clion teach me lifetime.
    pointer: usize,
}

fn main() {
    let input = env::args()
        .nth(1)
        .expect("Invalid");

    let mut state = State {
        stream: &input,
        pointer: 0,
    };

    println!("{}", eval(&mut state).unwrap());
}

fn eval(state: &mut State) -> Option<usize> {
    let c = state.stream
        .chars()
        .nth(state.pointer)?;

    state.pointer += 1;

    match c {
        '0'..='9' => {
            let num = c.to_digit(10)?;
            return Some(num as usize); // require explicitly convert
        }
        _ => {
            return None;
        }
    }
}

