use std::env::args;

mod files;
mod console;

fn main () {
    let argv: Vec<String> = args().collect(); 
    let argc = args().len();
    let text = files::read("example1.nate".to_string()).unwrap();
    console::print_vs(argv);
    println!("{}", argc);
    console::print_v(text.array);
    console::print(text.content.clone());
}
