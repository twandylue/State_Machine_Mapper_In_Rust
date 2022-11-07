use state_machine_mapper::{formate_output, run, StateMachine};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);
    match run(&args[1]) {
        Ok(content) => {
            let s = StateMachine::build(&content);
            // TODO:
            // 1. print stdout in more detail.
            println!("states list: {:#?}", s.states());
            println!("commands list: {:#?}", s.commands());
            println!("states and commands map: \n{:?}", s.map());
            println!();
            println!("* Output: ");
            formate_output(&s);
        }
        Err(err) => println!("Application error: {}", err),
    }
}
