use state_machine_mapper::{run, StateMachine};

fn main() {
    match run("test.csv") {
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
