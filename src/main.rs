/*
MIT License

Copyright (c) 2021 Pedro M. Torruella N.
*/

use serde::{Deserialize, Serialize};
use serde::ser::{Serializer, SerializeStruct, SerializeSeq};

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

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
        // 1) Need to count the number of fields we will have on the
        //    main branch.
        //
        // 3 is the number of fields in the struct.
        let mut seq = serializer.serialize_seq(Some(6))?;
        let version = ("edifversion", "2", "0", "0");
        let level = ("edifLevel", 0);
        let kword = ("keywordmap", ("keywordlevel", 0));

        seq.serialize_element("edif")?;
        seq.serialize_element(&self.design_name)?;
        seq.serialize_element(&version)?;
        seq.serialize_element(&level)?;
        seq.serialize_element(&kword)?;

        let mytup = ("Library", &self.libraries);

        seq.serialize_element(&mytup)?;

        seq.end()
    }
}

fn main() {
    let mut point = Edif {
        design_name: "dsp_2".to_string(),
        libraries: Vec::new(),
        comments: Vec::new(),
        designs: Vec::new()
    };

    point.libraries.push(5);
    point.libraries.push(3);
    point.libraries.push(2);

    let serialized = serde_sexpr::to_string(&point).unwrap();

    println!("serialized = {}", serialized);

    let path = Path::new("test.edf");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(serialized.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }

    // Convert the JSON string back to a Point.
    //let deserialized: Edif = serde_sexpr::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    //println!("deserialized = {:?}", deserialized);
}
