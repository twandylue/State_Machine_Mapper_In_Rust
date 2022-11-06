use state_machine_mapper::{run, StateMachine};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);
    match run(&args[1]) {
        Ok(content) => {
            let s = StateMachine::build(&content);
            // TODO: print stdout in more detail.
            println!("states list: {:#?}", s.states());
            println!("commands list: {:#?}", s.commands());
            println!("states and commands map: {:?}", s.map());
        }
        Err(err) => println!("Application error: {}", err),
    }
}
