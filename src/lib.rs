/*
MIT License

Copyright (c) 2021 Pedro M. Torruella N.
*/

use serde::{Serialize};
use serde::ser::{Serializer, SerializeSeq};



/*
#[derive(Debug)]
pub enum DirectionType {
    Input,
    Output
}


#[derive(Debug)]
pub struct InterfaceElem {
    name: String,
    direction: DirectionType
}*/

/*
#[derive(Debug)]
pub struct Cell {
    name: String,
    view: String,
    //interface: Vec<InterfaceElem> <<< this is wrong!
}*/

#[derive (Debug)]
pub struct Library {
        name: String,
        //technology: String,
        elem: String,
}

impl Serialize for Library {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {    
        let mut seq = serializer.serialize_seq(Some(2))?;

        seq.serialize_element("Library")?;
        seq.serialize_element(&self.name)?;
        seq.serialize_element(&self.elem)?;
        seq.end()
    }        
}


#[derive(Debug)]
pub enum EdifElements {
    Library(Library),
    Design(String)
}



// shorten names
#[derive(Debug)]
pub struct Edif {
    pub design_name: String,
    pub libraries: Vec<u8>,
    pub comments: Vec<u8>,
    pub designs: Vec<u8>,
    //pub foo: Bar,
}


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

        if  self.libraries.len() != 0 {
            seq.serialize_element(&self.libraries)?;
        }


        seq.end()
    }
}