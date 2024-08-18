use::std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let river = &args[1];
    let section = &args[2];

    println!("The river is {river} and the section is {section}")
}
