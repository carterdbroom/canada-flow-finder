use::std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    
    
    // Response
    println!("The river flow on the {river} at {section} is {flow}cms today🌊");
    println!("Do you want to look another river up? [y/n]");
    //Case of y
    println!("Enter the river and the station");

    // Case of n
    println!("Have fun on the river!");

    println!("The river is {river} and the section is {section}");
}

fn parse_config(args: &[String]) -> Config {
    let river = &args[1];
    let section = &args[2];
    Config {river, section}
}

struct Config {
    river: String,
    section: String

}