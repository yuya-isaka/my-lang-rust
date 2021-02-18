use std::env;
use std::process;

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

    println!("{:?}", eval(&mut state).unwrap());
}

fn eval(state: &mut State) -> Option<usize> {

    let c = state.stream
        .chars()
        .nth(state.pointer)?;

    state.pointer += 1;

    match c {
        _ if c.is_whitespace() => {
            return eval(state);
        }
        '0'..='9' => {
            let mut num = c.to_digit(10)?;
            while state.pointer <= state.stream.len() - 1 && state.stream.chars().nth(state.pointer)?.is_digit(10) { // ここではis_digitでbool返す
                num = num * 10 + state.stream.chars().nth(state.pointer)?.to_digit(10)?;
                state.pointer += 1;
            }
            return Some(num as usize); // require explicitly convert
        }
        '+' | '-' | '*' | '/' => {
            let num_1 = eval(state)?;
            let num_2 = eval(state)?;
            let result = match c {
                '+' => num_1 + num_2,
                '-' => num_1 - num_2,
                '*' => num_1 * num_2,
                '/' => num_1 / num_2,
                _ => error(format!("Invalid operator: {:?}", c)),
            };
            return Some(result);
        }
        _ => error(format!("Invalid character: {:?}, {:?}", c, state.pointer)),
    }
}

fn error(err_script: String) -> ! {
    eprintln!("{}", err_script);
    process::exit(1);
}

