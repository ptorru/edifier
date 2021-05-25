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
struct writtenStatus {
    timestamp: (u32,u32),
    program: String
}*/

#[derive(Serialize, Deserialize, Debug)]
enum status {
    writtenStatus{
        timestamp: (u32,u32),
        program: String
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct xilinx_edif {
    design_name: String,
    design_status: status
}

fn main() {
    //let wrstat = writtenStatus {timestamp: (2020,9), program: "Program Pedro".to_string()};
    let stt = status::writtenStatus {timestamp: (2020,9), program: "Program Pedro".to_string()};
    let point = xilinx_edif {design_name: "Design name".to_string(), design_status: stt};

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: xilinx_edif = serde_json::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);
}