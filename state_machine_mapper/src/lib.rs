use std::{collections::HashMap, error::Error, fs};

pub struct StateMachine {
    states: HashMap<String, i32>,
    commands: HashMap<String, i32>,
    map: Vec<Vec<i32>>,
}

pub fn run(input_file: &str) -> Result<String, Box<dyn Error>> {
    return Ok(fs::read_to_string(input_file)?);
}

impl StateMachine {
    pub fn states(&self) -> &HashMap<String, i32> {
        &self.states
    }

    pub fn commands(&self) -> &HashMap<String, i32> {
        &self.commands
    }

    pub fn map(&self) -> &Vec<Vec<i32>> {
        &self.map
    }

    pub fn build(content: &str) -> Self {
        let mut state_machine = StateMachine {
            states: HashMap::new(),
            commands: HashMap::new(),
            map: Vec::new(),
        };

        let mut i = 0;
        content.split("\n").for_each(|x| {
            let mut pre_state: String = String::new();
            let mut command: String = String::new();
            x.split(",").for_each(|x| {
                let commands_len = state_machine.commands.len() as i32;
                let states_len = state_machine.states.len() as i32;
                if x.is_empty() {
                    return;
                }

                if i % 3 == 1 {
                    command = x.trim().to_string();
                    state_machine
                        .commands
                        .entry(command.to_string())
                        .or_insert(commands_len);
                } else if i % 3 == 0 {
                    pre_state = x.trim().to_string();
                    state_machine
                        .states
                        .entry(pre_state.to_string())
                        .or_insert(states_len);
                } else if i % 3 == 2 {
                    let s = state_machine.states[&pre_state] as usize;
                    let c = state_machine.commands[&command] as usize;

                    let after_state = x.trim().to_string();
                    state_machine
                        .states
                        .entry(after_state.to_string())
                        .or_insert(states_len);

                    // NOTE: dynamically generate two dimension Vector to store info about relation
                    // between state and command.
                    // TODO: Maybe be able to use dynamic programming?
                    while state_machine.map.len() < state_machine.states.len() {
                        state_machine.map.push(Vec::new());
                    }
                    for i in 0..state_machine.map.len() {
                        while state_machine.map[i].len() < state_machine.commands.len() {
                            state_machine.map[i].push(-1);
                        }
                    }

                    if state_machine.map[s][c] != -1 {
                        let org_state = state_machine.states.iter().find_map(|(key, value)| if *value == state_machine.map[s][c] {Some(key)} else {None});
                        panic!("Error: previous state: {} receiving the same command: {} becomes different states: {} || {}", pre_state, command, org_state.unwrap_or(&"None".to_string()), after_state);
                    }
                    state_machine.map[s][c] = state_machine.states[&after_state];
                }
                i += 1;
            })
        });

        return state_machine;
    }
}

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

        let s = StateMachine::build(&content);

        assert_eq!(
            *s.states(),
            HashMap::from([
                ("state_a".to_string(), 0),
                ("state_b".to_string(), 1),
                ("state_c".to_string(), 2)
            ])
        );
        assert_eq!(
            *s.commands(),
            HashMap::from([("command_1".to_string(), 0), ("command_2".to_string(), 1),])
        );

        assert_eq!(s.states["state_a"].to_owned(), 0);
        assert_eq!(s.states["state_b"].to_owned(), 1);
        assert_eq!(s.states["state_c"].to_owned(), 2);
        assert_eq!(s.commands["command_1"].to_owned(), 0);
        assert_eq!(s.commands["command_2"].to_owned(), 1);
    }

    #[test]
    fn state_machine_map_ok() {
        let content = String::from(
            "
state_a, command_1, state_b\n
state_a, command_2, state_c\n
state_b, command_1, state_c\n
state_b, command_2, state_d\n
state_c, command_3, state_d\n
",
        );

        let s = StateMachine::build(&content);

        let actual = s.map();

        let expected = vec![
            vec![s.states["state_b"], s.states["state_c"], -1],
            vec![s.states["state_c"], s.states["state_d"], -1],
            vec![-1, -1, s.states["state_d"]],
            vec![-1, -1, -1],
        ];

        assert_eq!(*actual, expected);
    }

    #[test]
    #[should_panic(
        expected = "Error: previous state: state_a receiving the same command: command_2 becomes different states: state_c || state_d"
    )]
    fn state_machine_map_panic() {
        // NOTE: the same state receives the same command but becomes different final state, this
        // case should be panic
        let content = String::from(
            "
state_a, command_1, state_b\n
state_a, command_2, state_c\n
state_a, command_2, state_d\n
state_b, command_1, state_c\n
state_b, command_2, state_d\n
state_c, command_3, state_d\n
",
        );

        StateMachine::build(&content);
    }
}
