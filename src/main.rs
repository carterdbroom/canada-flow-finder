use std::io;
use serde::{Serialize, Deserialize};
use serde_json::from_str;
use reqwest::blocking::Client;
use serde_json::Error;
use chrono::{self, Days};


#[derive(Serialize, Deserialize, Debug)]
struct River {
    province: String,
    operations: String,
    name: String,
    latlng: Vec<f64>,
    #[serde(rename(deserialize = "6hrs_data"))]
    six_hrs_data: String,
    id: String 
}
#[derive(Serialize, Deserialize, Debug)]
struct RiverList {
    code: i32,
    details: String,
    message: Vec<River>
}

// When getting flow documentation
#[derive(Serialize, Deserialize, Debug)]
struct FlowCall {
    code: i32,
    details: String,
    message: FlowData
}
// This is the struct for the data in the message attribute of the FlowCall struct. 
#[derive(Serialize, Deserialize, Debug)]
struct FlowData {
    #[serde(rename(deserialize = "startDate"))]
    start_date: String,
    #[serde(rename(deserialize = "endDate"))]
    end_date: String,
    unit: String, 
    history: Vec<Flow>
}
// A struct that represents a piece of FlowDat
#[derive(Serialize, Deserialize, Debug)]
struct Flow {
    date: String, 
    value: String
}
// Put your API key here
const KEY: &str = "-O5A-mmDjkK19KriaFk0";
fn main() {
    // Creating a reqwest client.
    let client: Client = Client::new();
    
    // A boolean for the while loop for the program.
    let mut program_running: bool = true;
    
    // Getting a json of rivers, and then deserializing it into a vector of Rivers.
    let river_list: Vec<River> = deserialize_river_list(get_river_list(&client).as_str());
    
    // The loop that continuously runs while the user is using the tool.
    while program_running {
        println!("Enter the name of the river or creek you would like to get data for: ");  
        
        // Getting input from the user for the river they would like to get data from.
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        // Creating a vector that holds river search matches.
        let mut matches = Vec::new();
        
        // Counter for number of matches.
        let mut counter: u32 = 0;
        
        println!("Here are the results:\n");

        // Searching through the list of rivers and displaying the matches numbered.
        for river in &river_list {
            if river.name.contains(input.to_uppercase().trim()) {
                counter += 1;
                matches.push((&river.name, &river.id));
                println!("[{}] {}", counter, &river.name);
            }
        }

        if matches.len() > 0 {  
            loop {
                println!("\nEnter the number next to the station you would like to choose:");
                
                // Taking input for the user's choice.
                let mut number = String::new();
                io::stdin().read_line(&mut number).unwrap();
                
                // Match statement to see if the user entered a valid usize.
                match number.trim().parse::<usize>() {
                    Ok(n) => {
                        // Making sure the user enters a number in a valid range.
                        if n <= matches.len() && n > 0 {
                            // Getting the associated river id from the name.
                            let chosen_river_id: &String = &matches[n-1].1;
                            
                            // Getting flow data and then deserializing it into a vector of Flow structs.
                            let flow_data_list = deserialize_river_flow(get_river_flow(&client, &chosen_river_id).as_str());
                            
                            // Getting the last element in the vector, which is associated with the latest real-time piece of data.
                            let last_element: usize = flow_data_list.len() - 1;
                            
                            // Getting the latest flow value.
                            let latest_river_flow = &flow_data_list[last_element].value;
                            
                            // Getting the data associated with the latest flow value.
                            let date = &flow_data_list[last_element].date;
                            
                            // Displaying all data to the user.
                            println!("Displaying Data:\n");
                            println!("Station Name: {} 🚧", &matches[n-1].0);
                            println!("Date: {} 📅", date);
                            println!("River Flow: {} cubic metres per second 🌊\n", &latest_river_flow);
                            println!("Would you like to get data for another river? [y/n]");
                            
                            // Input for whether the user wants to get data for another river.
                            let mut answer = String::new();
                            io::stdin().read_line(&mut answer).unwrap();
                            if answer.trim().eq_ignore_ascii_case("n") {
                                program_running = false;
                            }
                            break;
                        }
                        else {
                            println!("Your number was either too big or too small!");
                        }
                        
                    },
                    Err(_) => {
                        println!("You entered a non-valid number!")
                    }
                }    
            }
        }
        else {
            println!("Unable to find such a river in the database.");
            println!("Would you like to search for another river? [y/n]");

            // Input for whether the user wants to get data for another river.
            let mut answer = String::new();
            io::stdin().read_line(&mut answer).unwrap();

            if answer.trim().eq_ignore_ascii_case("n") {
                program_running = false;
            }
        }
    }    
    println!("Have fun on the river! 🤙")
}


// This function gets a String of all the rivers from Enrivonment Canada's hydrometric database.
fn get_river_list (client: &Client) -> String {
    let url = format!("https://vps267042.vps.ovh.ca/scrapi/stations?page=1,2,3,4,5,6,7,8,9,10,11counter&key={}", KEY);
    
    // Getting data river list data.
    let river_list = client.get(&url).send();

    // Match statement in case getting data is unsuccessful.
    match river_list {
        Ok(l1) => {
            // Converting the Response to text.
            match l1.text() {
                Ok(l2) => {
                    l2
                },
                Err(_) => {
                    panic!("Can't convert river list to text.")
                }
            }
        },
        Err(_) => {
            panic!("Error getting river list.")
        }
    }
}

// This function deserializes the river list json data.
fn deserialize_river_list(json_string: &str) -> Vec<River> {
    // Deserializes the json into the RiverList struct using serde.
    let river_list: Result<RiverList, Error> = from_str::<RiverList>(json_string);
   
    // Match statement incase deserializing isn't successful.
    match river_list {
        Ok(list) => list.message,
        Err(e) => {
            println!("{:?}", e);
            panic!("Error deserializing json for list of rivers");
        }
    }
}

// This function gets a String of the flow data for a river with the associated id.
fn get_river_flow (client: &Client, id: &String) -> String {
    let url: String = format!("https://vps267042.vps.ovh.ca/scrapi/station/{}/flow/?startDate={}&endDate={}&resultType=history&key={}", id, chrono::offset::Local::now().checked_sub_days(Days::new(1)).unwrap().format("%Y-%m-%d").to_string(), chrono::offset::Local::now().format("%Y-%m-%d").to_string(), KEY);
    
    // Getting the river flow data. 
    let flow_data = client.get(url).send();

    // Match statement in case getting flow data is unsuccessful.
    match flow_data {
        Ok(flow) => {
            // Converting the Response to text.
            match flow.text() {
                Ok(f) => f,
                Err(e) => {
                    println!("{:?}", e);
                    panic!("Can't convert river flow to text.")        
                }
            }
        },
        Err(e) => {
            println!("{:?}", e);
            panic!("Error getting river flow.")
        }
    } 
}

// This function deserializes the river flow data.
fn deserialize_river_flow (json_string: &str) -> Vec<Flow>{
    //Deserializes the json into FlowCal struct using serde. 
    let river_flow_list = from_str::<FlowCall>(json_string);
    
    // Match statement in case deserializing isn't successful.
    match river_flow_list {
        Ok(flow_list) => {
            // The message is a FlowData struct and the history is a Flow struct.
            flow_list.message.history
        }
        Err(e) => {
            println!("{:?}", e);
            panic!("Can't deserialize json for river flow")
        }
    }
}