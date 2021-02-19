use std::env;
use std::process;
use std::collections::HashMap;

struct State<'a> {
    stream: &'a str, // Clion teach me lifetime.
    pointer: usize,
    func: HashMap<char, String>,
}

fn main() {
    let input = env::args()
        .nth(1)
        .expect("Invalid");

    let hs = HashMap::new();
    let mut state = State {
        stream: &input,
        pointer: 0,
        func: hs,
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
        'A'..='Z' if state.stream.chars().nth(state.pointer)? == '[' => {
            let name = c;
            let mut define = "".to_string();
            state.pointer += 1;
            while state.stream.chars().nth(state.pointer)? != ']' {
                // if state.stream.chars().nth(state.pointer)?.is_whitespace() {
                //     state.pointer += 1;
                //     continue;
                // }
                define.push(state.stream.chars().nth(state.pointer)?);
                state.pointer += 1;
            }

            // println!("{:?}", define);
            // if define.is_empty() {
            //     state.pointer += 1;
            //     return eval(state);
            // }
            state.func.insert(name, define);
            state.pointer += 1;
            return eval(state);
        }
        'A'..='Z' if state.stream.chars().nth(state.pointer)? == '(' => {
            let name = c;
            state.pointer += 1;

            let define = state.func.get(&name)?;
            let mut func_pointer = 0;
            let mut val = 0;
            while func_pointer <= define.len() - 1 {
                let mut func_state = State {
                    stream: define,
                    pointer: func_pointer,
                    func: state.func.clone(),
                };
                let result = eval(&mut func_state)?;
                val = result;
                func_pointer = func_state.pointer;
                while func_pointer <= define.len() - 1 && func_state.stream.chars().nth(func_pointer)?.is_whitespace() {
                    func_pointer += 1;
                }
            }
            state.pointer += 1;
            return Some(val);
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

