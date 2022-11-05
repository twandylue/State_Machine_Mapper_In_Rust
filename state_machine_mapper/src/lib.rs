use std::{collections::HashMap, error::Error, fs};

pub struct StateMachine {
    states: HashMap<String, i32>,
    commands: HashMap<String, i32>,
}

fn run(input_file: &str) -> Result<String, Box<dyn Error>> {
    return Ok(fs::read_to_string(input_file)?);
}

impl StateMachine {
    pub fn build(content: String) -> Self {
        let mut state_machine = StateMachine {
            states: HashMap::new(),
            commands: HashMap::new(),
        };

        content.split("\n").for_each(|x| {
            let mut i = 0;
            x.split(",").for_each(|x| {
                if !x.is_empty() {
                    if i == 1 {
                        // TODO: need to be more concise
                        match state_machine.commands.get(&x.trim().to_string()) {
                            Some(_) => (),
                            None => {
                                state_machine.commands.insert(
                                    x.trim().to_string(),
                                    (state_machine.commands.len() + 1) as i32,
                                );
                            }
                        }
                    } else {
                        match state_machine.states.get(&x.trim().to_string()) {
                            Some(_) => (),
                            None => {
                                state_machine.states.insert(
                                    x.trim().to_string(),
                                    (state_machine.states.len() + 1) as i32,
                                );
                            }
                        }
                    }
                    i += 1;
                }
            })
        });

        return state_machine;
    }
}

// TODO:
// fn state_mapper(pre_state: &str, command: &str, after_state: &str) -> Vec<Vec<u32>> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_machine_build_ok() {
        let content = String::from(
            "
state_a, command_1, state_b\n
state_a, command_2, state_c\n
state_b, command_1, state_c\n
",
        );

        let s = StateMachine::build(content);

        assert_eq!(
            s.states,
            HashMap::from([
                ("state_a".to_string(), 1),
                ("state_b".to_string(), 2),
                ("state_c".to_string(), 3)
            ])
        );
        assert_eq!(
            s.commands,
            HashMap::from([("command_1".to_string(), 1), ("command_2".to_string(), 2),])
        );

        assert_eq!(s.states.get(&"state_a".to_string()).unwrap().to_owned(), 1);
        assert_eq!(s.states.get(&"state_b".to_string()).unwrap().to_owned(), 2);
        assert_eq!(s.states.get(&"state_c".to_string()).unwrap().to_owned(), 3);
        assert_eq!(
            s.commands.get(&"command_1".to_string()).unwrap().to_owned(),
            1
        );
        assert_eq!(
            s.commands.get(&"command_2".to_string()).unwrap().to_owned(),
            2
        );
    }
}
