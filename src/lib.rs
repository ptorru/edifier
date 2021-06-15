/*
MIT License

Copyright (c) 2021 Pedro M. Torruella N.
*/

use serde::{Serialize};
use serde::ser::{Serializer, SerializeSeq};




/* contents -> instance
            -> net

            */

#[derive(Debug)]
pub enum PortDirection {
    Input,
    Output,
}

impl Serialize for PortDirection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {    
        match self {
            PortDirection::Input => "INPUT".serialize(serializer),
            PortDirection::Output => "OUTPUT".serialize(serializer)
        }
    }
}

#[derive(Debug)]
pub enum PortElements {
    Direction(PortDirection),
    //Array(PortArray)
}

impl Serialize for PortElements {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {    
        match self {
            PortElements::Direction(dir) => dir.serialize(serializer)
        }
    }        
}


#[derive(Debug)]
pub struct InterfacePort {
    pub name: String,
    pub element: PortElements,
}

impl Serialize for InterfacePort {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {    
        let mut seq = serializer.serialize_seq(Some(3))?;
        seq.serialize_element(&"port".to_string())?;
        seq.serialize_element(&self.name)?;
        seq.serialize_element(&self.element)?;
        seq.end()
    }        
}


#[derive(Debug)]
pub struct CellInterface {
    pub ports: Vec<InterfacePort>,
}

impl Serialize for CellInterface {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {    
        if  !self.ports.is_empty() {
            let mut seq = serializer.serialize_seq(Some(1))?;
            seq.serialize_element(&self.ports)?;
            seq.end()
        } else {
            serializer.serialize_none()
        }
    }        
}

#[derive(Debug)]
pub struct CellView {
    pub name: String,
    pub interfaces: Vec<u8>,//(InterfaceElem),
    pub contents: Vec<u8>,//(CellContents),
}

impl Serialize for CellView {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {    
        let mut seq = serializer.serialize_seq(Some(4))?;
        let viewtype = ("viewtype", "NETLIST");

        seq.serialize_element("view")?;
        seq.serialize_element(&self.name)?;
        seq.serialize_element(&viewtype)?;
        if  !self.interfaces.is_empty() {
            seq.serialize_element(&self.interfaces)?;
        }
        if  !self.contents.is_empty() {
            seq.serialize_element(&self.contents)?;
        }
        seq.end()
    }        
}

#[derive(Debug)]
pub struct Cell {
    pub name: String,
    pub views: Vec<CellView>,
}

impl Serialize for Cell {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {    
        let mut seq = serializer.serialize_seq(Some(4))?;
        let celltype = ("celltype", "GENERIC");

        seq.serialize_element("cell")?;
        seq.serialize_element(&self.name)?;
        seq.serialize_element(&celltype)?;
        if  !self.views.is_empty() {
            seq.serialize_element(&self.views)?;
        }
        seq.end()
    }        
}

#[derive (Debug)]
pub struct Library {
        pub name: String,
        pub elements: Vec<Cell>,
}

impl Serialize for Library {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {    
        let mut seq = serializer.serialize_seq(Some(3))?;

        seq.serialize_element("Library")?;
        seq.serialize_element(&self.name)?;
        if  !self.elements.is_empty() {
            seq.serialize_element(&self.elements)?;
        }
        seq.end()
    }        
}


#[derive(Debug)]
pub enum EdifElements {
    Library(Library),
    //Design(String)
}

impl Serialize for EdifElements {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {    
        match self {
            EdifElements::Library(lib) => lib.serialize(serializer)
        }
    }        
}


#[derive(Debug)]
pub struct Edif {
    pub name: String,
    pub elements: Vec<EdifElements>,
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
        seq.serialize_element(&self.name)?;
        seq.serialize_element(&version)?;
        seq.serialize_element(&level)?;
        seq.serialize_element(&kword)?;

        if  !self.elements.is_empty() {
            seq.serialize_element(&self.elements)?;
        }


        seq.end()
    }
}