/*
MIT License

Copyright (c) 2021 Pedro M. Torruella N.
*/
use std::path::Path;

use std::fs::File;
use std::io::Write;



static DOFILE: bool = false;

fn main() {

    let point = edifier::Edif {
        name: "dsp_2".to_string(),
        elements: Vec::new(),
    };

    //point.libraries.push(mylib);

    let serialized = serde_sexpr::to_string(&point).unwrap();

    println!("serialized = {}", serialized);

    let path = Path::new("test.edf");
    let display = path.display();

    if DOFILE {
    // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        match file.write_all(serialized.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }
    }
    
}
