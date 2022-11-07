use state_machine_mapper::{
    formate_output, output_in_cpp, output_in_csharp, output_in_go, output_in_js, output_in_python,
    output_in_rust, run, StateMachine,
};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);
    match run(&args[1]) {
        Ok(content) => {
            let s = StateMachine::build(&content);
            // TODO:
            // 1. print stdout in more detail.
            // 2. generate list in different languages
            println!("states list: {:#?}", s.states());
            println!("commands list: {:#?}", s.commands());
            println!();

            println!("* === Output ===");
            formate_output(&s);
            println!();

            println!("* === Map in different languages ===");
            println!("Map in rust: \n{}", output_in_rust(&s));
            println!("Map in cpp: \n{}", output_in_cpp(&s));
            println!("Map in go: \n{}", output_in_go(&s));
            println!("Map in c_sharp: \n{}", output_in_csharp(&s));
            println!("Map in js: \n{}", output_in_js(&s));
            println!("Map in python: \n{}", output_in_python(&s));
        }
        Err(err) => println!("Application error: {}", err),
    }
}
