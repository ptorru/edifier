/*
MIT License

Copyright (c) 2021 Pedro M. Torruella N.
*/

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
enum Status {
    WrittenStatus {
        timestamp: (u32, u32),
        program: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
enum CellType {
    Generic,
}

// change type for ty
#[derive(Serialize, Deserialize, Debug)]
struct CellElement {
    Name: String,
    Type: CellType,
}

#[derive(Serialize, Deserialize, Debug)]
enum LibraryElement {
    Cell(CellElement),
}

// Hacer library como otra struct
// Put elements as lower case
// use LibraryTy

Library {
    Name: String,
    EdifLevel: u32,
    Technology: String,
    Cells: Vec<CellElement>,
    Netlist: Netlist
},

#[derive(Serialize, Deserialize, Debug)]

enum EdifElements {
    Library {
        Name: String,
        EdifLevel: u32,
        Technology: String,
        Elements: Vec<LibraryElement>,
    },
    Comment(String),
}

// shorten names
#[derive(Serialize, Deserialize, Debug)]
struct Edif {
    design_name: String,
    design_status: Status,
    Elements: Vec<EdifElements>,
}

fn main() {
    //let wrstat = WrittenStatus {timestamp: (2020,9), program: "Program Pedro".to_string()};
    let stt = Status::WrittenStatus {
        timestamp: (2020, 9),
        program: "Program Pedro".to_string(),
    };
    let cell = CellElement {
        Name: "my cell".to_string(),
        Type: CellType::Generic,
    };
    let cell = LibraryElement::Cell(cell);
    let lib = EdifElements::Library {
        Name: "my lib".to_string(),
        EdifLevel: 1,
        Technology: "NumberDefinition".to_string(),
        Elements: vec![cell],
    };
    let com = EdifElements::Comment("this is a comment for thid design".to_string());
    let point = Edif {
        design_name: "Design name".to_string(),
        design_status: stt,
        Elements: vec![lib, com],
    };

    // Convert the Point to a JSON string.
    //let serialized = serde_json::to_string(&point).unwrap();
    let serialized = serde_sexpr::to_string(&point).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: Edif = serde_sexpr::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);
}
