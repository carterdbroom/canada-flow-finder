//use std::io;
use serde::{Serialize, Deserialize};
use serde_json::from_str;
use reqwest::blocking::Client;
use serde_json::Error;


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

const KEY: &str = "";
fn main() {
    
    let client: Client = Client::new();
    
    //println!("This is the response I get: {}", get_river_list(&client));

    deserialize_river_list(get_river_list(&client).as_str());
}

fn get_river_list (client: &Client) -> String {
    let url = format!("https://vps267042.vps.ovh.ca/scrapi/stations?page=1,2,3,4,5,6,7,8,9,10,11counter&key={}", KEY);
    let river_list = client
        .get(&url)
        .send();

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

//fn deserialize_river_list(json_string: &str) -> Vec<River> {
  //  let river_deserialization = from_str::<Vec<River>>(json_string);

    //match river_deserialization {
      ///  Ok(rivers) => rivers,
        //Err(e) => {
          //  println!("Error deserializing JSON: {:?}", e);
            //panic!("Error deserializing json for list of rivers");
        //}
    //}
//}

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