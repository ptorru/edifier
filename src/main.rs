/*
MIT License

Copyright (c) 2021 Pedro M. Torruella N.
*/

use serde::{Deserialize, Serialize};
use serde::ser::{Serializer, SerializeStruct};

/*
#[derive(Serialize, Deserialize, Debug)]
enum EdifElements {
    Library {
        name: String,
        edif_evel: u8,
        technology: String,
        elements: Vec<i32>,
    },
    Comment(String),
    Design(String)
}*/

// shorten names
#[derive(Deserialize, Debug)]
struct Edif {
    design_name: String,
    design_status: String,
    /*    libraries: Vec<EdifElements::Library>,
    comments: Vec<EdifElements::Comment>,
    design: Vec<EdifElements::Elements>*/
}

impl Serialize for Edif {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Edif", 2)?;
        state.serialize_field("design_name", &self.design_name)?;
        state.serialize_field("design_status", &self.design_status)?;
        state.end()
    }
}

fn main() {
    let point = Edif {
        design_name: "Design name".to_string(),
        design_status: "Design status".to_string(),
    };

    let serialized = serde_sexpr::to_string(&point).unwrap();

    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: Edif = serde_sexpr::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);
}
