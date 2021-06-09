/*
MIT License

Copyright (c) 2021 Pedro M. Torruella N.
*/

use serde::{Serialize};
use serde::ser::{Serializer, SerializeSeq};

use std::path::Path;

use std::fs::File;
use std::io::Write;

static NETLIST: &str = "netlist";
static DOFILE: bool = false;

#[derive(Debug)]
enum DirectionType {
    Input,
    Output
}


#[derive(Debug)]
struct InterfaceElem {
    name: String,
    direction: DirectionType
}

#[derive(Debug)]
struct Cell {
    name: String,
    view: String,
    //interface: Vec<InterfaceElem> <<< this is wrong!
}

// pasar a su propio struct y tener los named typs en el enum
/*pub enum EdifElem {
    Library(Library),
    ...
  }*/
#[derive(Debug)]
enum EdifElements {
    Library {
        name: String,
        //technology: String,
        elem: Vec<Cell>,
    },
    Design(String)
}

#[derive(Debug)]
pub struct Bar {
    name: String
}

impl Serialize for Bar{
    
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {    
        let mut seq = serializer.serialize_seq(Some(2))?;

        seq.serialize_element("name")?;
        seq.serialize_element(&self.name)?;
        seq.end()
    }
}

// shorten names
#[derive(Debug)]
struct Edif {
    design_name: String,
    libraries: Vec<u8>,
    comments: Vec<u8>,
    designs: Vec<u8>,
    foo: Bar,
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

        seq.serialize_element(&self.foo)?;

        seq.end()
    }
}

fn main() {
    //let my_vec: Vec<T> = Vec::new();
    /*
    let mut inter: Vec<InterfaceElem> = Vec::new();
    inter.push(InterfaceElem::new("a", DirectionType::new(DirectionType::Input)));

    let mut mycell = Cell
 {
        name: "LUT2",
        view: NETLIST,
        interface: inter
    };

    let mut mycells: Vec<Cell> = Vec::new();
    mycells.push(mycell);

    let mut mylib = EdifElements::Library {
        name: "hdi_primitives".to_string(),
        technology: "numberDefinition".to_string(),
        elements: mycells
    };
*/
    let mut point = Edif {
        design_name: "dsp_2".to_string(),
        libraries: Vec::new(),
        comments: Vec::new(),
        designs: Vec::new(),
        foo: Bar{name: "BAAr".to_string()},
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
