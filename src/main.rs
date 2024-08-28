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

#[derive(Serialize, Deserialize, Debug)]
struct FlowCall {
    code: i32,
    details: String,
    message: FlowData
}

#[derive(Serialize, Deserialize, Debug)]
struct FlowData {
    #[serde(rename(deserialize = "startDate"))]
    start_date: String,
    #[serde(rename(deserialize = "endDate"))]
    end_date: String,
    unit: String, 
    history: Vec<Flow>
}

#[derive(Serialize, Deserialize, Debug)]
struct Flow {
    date: String, 
    value: String
}

const KEY: &str = "-O5A-mmDjkK19KriaFk0";
fn main() {
    
    let client: Client = Client::new();
    let mut program_running: bool = true;
    let river_list: Vec<River> = deserialize_river_list(get_river_list(&client).as_str());
    //println!("{}", get_river_flow(&client, &id).as_str());
    while program_running {
        println!("Enter the name of the river or creek you would like to get data for: ");  

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut matches = Vec::new();
        let mut counter: u32 = 0;
        println!("Here are the results:\n");
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
                
                let mut number = String::new();
                
                io::stdin().read_line(&mut number).unwrap();
                
                match number.trim().parse::<usize>() {
                    Ok(n) => {
                        if n <= matches.len() && n > 0 {
                            let chosen_river_id: &String = &matches[n-1].1;
                            let flow_data_list = deserialize_river_flow(get_river_flow(&client, &chosen_river_id).as_str());
                            let last_element: usize = flow_data_list.len() - 1;
                            let latest_river_flow = &flow_data_list[last_element].value;
                            let date = &flow_data_list[last_element].date;
                            println!("Displaying Data:\n");
                            println!("Station Name: {} 🚧", &matches[n-1].0);
                            println!("Date: {} 📅", date);
                            println!("River Flow: {} cubic metres per second 🌊\n", &latest_river_flow);
                            println!("Would you like to get data for another river? [y/n]");

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

            let mut answer = String::new();

            io::stdin().read_line(&mut answer).unwrap();

            if answer.trim().eq_ignore_ascii_case("n") {
                program_running = false;
            }
        }
    }    
    println!("Have fun on the river! 🤙")
}

fn get_river_list (client: &Client) -> String {
    let url = format!("https://vps267042.vps.ovh.ca/scrapi/stations?page=1,2,3,4,5,6,7,8,9,10,11counter&key={}", KEY);
    let river_list = client.get(&url).send();

    match river_list {
        Ok(l1) => {
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

fn deserialize_river_list(json_string: &str) -> Vec<River> {
    // Deserialize the JSON into the RiverList struct
    let river_list: Result<RiverList, Error> = from_str::<RiverList>(json_string);
    match river_list {
        Ok(list) => list.message,
        Err(e) => {
            println!("{:?}", e);
            panic!("Error deserializing json for list of rivers");
        }
    }
}

fn get_river_flow (client: &Client, id: &String) -> String {
    let url: String = format!("https://vps267042.vps.ovh.ca/scrapi/station/{}/flow/?startDate={}&endDate={}&resultType=history&key={}", id, chrono::offset::Local::now().checked_sub_days(Days::new(1)).unwrap().format("%Y-%m-%d").to_string(), chrono::offset::Local::now().format("%Y-%m-%d").to_string(), KEY);
    let flow_data = client.get(url).send();

    match flow_data {
        Ok(flow) => {
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

fn deserialize_river_flow (json_string: &str) -> Vec<Flow>{
    let river_flow_list = from_str::<FlowCall>(json_string);
    match river_flow_list {
        Ok(flow_list) => {
            flow_list.message.history
        }
        Err(e) => {
            println!("{:?}", e);
            panic!("Can't deserialize json for river flow")
        }
    }
}