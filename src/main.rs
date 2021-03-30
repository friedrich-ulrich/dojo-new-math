#[derive(Debug)]
struct State {
    last_number: u32,
    current_number: u32,
    add: bool,
    mult: bool,
}

fn calc(input: &str, start: usize) -> Option<(u32, usize)> {
    let mut state = State {
        last_number: 0,
        current_number: 0,
        add: true,
        mult: false,
    };
    let mut i = start;

    while i < input.len() {
        println!("i: {}, state: {:?}", i, state);
        match input.chars().nth(i).unwrap() {
            c @ '0'..='9' => {
                state.current_number = state.current_number * 10 + c.to_digit(10).unwrap()
            }
            '+' => {
                if state.add {
                    state.last_number = state.last_number + state.current_number;
                    state.current_number = 0;
                } else if state.mult {
                    state.last_number = state.last_number * state.current_number;
                    state.current_number = 0;
                    state.mult = false;
                    state.add = true;
                }
            }
            '*' => {
                if state.add {
                    state.last_number = state.last_number + state.current_number;
                    state.current_number = 0;
                    state.mult = true;
                    state.add = false;
                } else if state.mult {
                    state.last_number = state.last_number * state.current_number;
                    state.current_number = 0;
                }
            }
            '(' => {
                let (sub, next_i) = calc(input, i + 1).unwrap();
                state.current_number = sub;
                i = next_i;
            }
            ')' => {
                break;
            }
            _ => return None,
        }
        i += 1;
    }

    if state.add {
        state.last_number = state.last_number + state.current_number;
    } else if state.mult {
        state.last_number = state.last_number * state.current_number;
    }

    Some((state.last_number, i))
}

fn main() {
    println!("{:?}", calc("42*5*5", 0));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_number() {
        let result = calc("4", 0);
        assert_eq!(result, Some((4, 1)));
    }

    #[test]
    fn two_digit() {
        let result = calc("42", 0);
        assert_eq!(result, Some((42, 2)));
    }

    #[test]
    fn nonsense() {
        let result = calc("a", 0);
        assert_eq!(result, None);
    }

    #[test]
    fn add1() {
        let result = calc("42+5", 0);
        assert_eq!(result, Some((47, 4)));
    }

    #[test]
    fn add2() {
        let result = calc("42+5+5", 0);
        assert_eq!(result, Some((52, 6)));
    }

    #[test]
    fn mult1() {
        let result = calc("42*5", 0);
        assert_eq!(result, Some((210, 4)));
    }
    #[test]
    fn mult2() {
        let result = calc("42*5*5", 0);
        assert_eq!(result, Some((1050, 6)));
    }
    #[test]
    fn combi1() {
        let result = calc("42*5+5", 0);
        assert_eq!(result, Some((215, 6)));
    }
    #[test]
    fn combi2() {
        let result = calc("42+5*5", 0);
        assert_eq!(result, Some((235, 6)));
    }
    #[test]
    fn para1() {
        let result = calc("42+(5*5)", 0);
        assert_eq!(result, Some((67, 8)));
    }
    #[test]
    fn para2() {
        let result = calc("42*(4+(5*5))", 0);
        assert_eq!(result, Some((1218, 12)));
    }
}
