/*
MIT License

Copyright (c) 2021 Pedro M. Torruella N.
*/

use serde::{Deserialize, Serialize};
use serde::ser::{Serializer, SerializeStruct, SerializeSeq};


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
    libraries: Vec<u8>,
    comments: Vec<u8>,
    designs: Vec<u8>
}

/*
impl<T> Serialize for Vec<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self {
            seq.serialize_element(e)?;
        }
        seq.end()
    }
}*/

impl Serialize for Edif {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut seq = serializer.serialize_seq(Some(3))?;
        let version = ("edifversion", "2", "0", "0");
        let level = ("edifLevel", 0);
        let ediftup = ("edif" , &self.design_name , version, level);
        seq.serialize_element(&ediftup)?;
        let mytup = ("library", &self.libraries);
        seq.serialize_element(&mytup)?;
        //seq.end();
        seq.end()
    }
}

fn main() {
    let mut point = Edif {
        design_name: "Design name".to_string(),
        design_status: "Design status".to_string(),
        libraries: Vec::new(),
        comments: Vec::new(),
        designs: Vec::new()
    };

    point.libraries.push(5);
    point.libraries.push(3);
    point.libraries.push(2);

    let serialized = serde_sexpr::to_string(&point).unwrap();

    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    //let deserialized: Edif = serde_sexpr::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    //println!("deserialized = {:?}", deserialized);
}
