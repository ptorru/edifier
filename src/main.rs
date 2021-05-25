/*
MIT License

Copyright (c) 2021 Pedro M. Torruella N.
*/

/*use::sexpr;

fn main() {
    println!("Hello, world!");
    let vec: Vec<i32> = vec![1,2,3];
    sexpr::encode(&vec)
}
*/


use serde::{Serialize, Deserialize};

/*
#[derive(Serialize, Deserialize, Debug)]
struct WrittenStatus {
    timestamp: (u32,u32),
    program: String
}*/

#[derive(Serialize, Deserialize, Debug)]
enum Status {
    WrittenStatus{
        timestamp: (u32,u32),
        program: String
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct XilinxEdif {
    design_name: String,
    design_status: Status
}

fn main() {
    //let wrstat = WrittenStatus {timestamp: (2020,9), program: "Program Pedro".to_string()};
    let stt = Status::WrittenStatus {timestamp: (2020,9), program: "Program Pedro".to_string()};
    let point = XilinxEdif {design_name: "Design name".to_string(), design_status: stt};

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: XilinxEdif = serde_json::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);
}