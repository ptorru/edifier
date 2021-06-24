/*
MIT License

Copyright (c) 2021 Pedro M. Torruella N.
*/

use serde::{Serialize};
use serde::ser::{Serializer, SerializeSeq};
use crate::ast::*;


impl Serialize for PortRef {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {    
        let mut seq = serializer.serialize_seq(Some(3))?;
        seq.serialize_element(&"portref".to_string())?;
        seq.serialize_element(&self.name)?;
        if  !self.instanceref.is_empty() {
            let instref = (&"instanceref".to_string(), &self.instanceref);
            seq.serialize_element(&instref)?;
        }
        
        seq.end()
    }        
}

impl Serialize for PortList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if  !self.is_empty() {

            let mut seq = serializer.serialize_seq(Some(self.len() + 1))?;
            seq.serialize_element(&"joined".to_string())?;
            for portref in self.iter() {
                seq.serialize_element(&portref)?;
            }
            seq.end()
        }
        else {
            serializer.serialize_none()
        }
    }
}

impl Serialize for ContentNet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {    
        let mut seq = serializer.serialize_seq(Some(3))?;
        seq.serialize_element(&"net".to_string())?;
        seq.serialize_element(&self.name)?;

        if  !self.portlist.is_empty() {
            seq.serialize_element(&self.portlist)?;
        }
        seq.end()
    }        
}

impl Serialize for ContentInstance {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {    
        let mut seq = serializer.serialize_seq(Some(3))?;
        let all = ("viewref", &self.viewref, 
                      ("cellref", &self.cellref, 
                      ("libraryref", &self.libraryref)));

        seq.serialize_element(&"instance".to_string())?;
        seq.serialize_element(&self.name)?;
        seq.serialize_element(&all)?;
        seq.end()
    }        
}

impl Serialize for ContentElement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {    
        let mut seq = serializer.serialize_seq(Some(2))?;
        seq.serialize_element(&"direction".to_string())?;
        match self {
            ContentElement::Instance  => {seq.serialize_element(&"instance".to_string())?;},
            ContentElement::Net => {seq.serialize_element(&"net".to_string())?;}
        }
        seq.end()
    }
}

impl Serialize for CellContents {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {    
        if  !self.is_empty() {
            let mut seq = serializer.serialize_seq(Some(2))?;
            seq.serialize_element(&"contents".to_string())?;

            for element in self.iter() {
                seq.serialize_element(&element)?;
            }
            seq.end()
        } else {
            serializer.serialize_none()
        }
    }        
}
    
impl Serialize for PortDirection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {    
        let mut seq = serializer.serialize_seq(Some(2))?;
        seq.serialize_element(&"direction".to_string())?;
        match self {
            PortDirection::Input  => {seq.serialize_element(&"INPUT".to_string())?;},
            PortDirection::Output => {seq.serialize_element(&"OUTPUT".to_string())?;}
        }
        seq.end()
    }
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

impl Serialize for CellInterface {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {    
        if  !self.is_empty() {
            let mut seq = serializer.serialize_seq(Some(1 + self.len()))?;
            seq.serialize_element(&"interface".to_string())?;

            for port in self.iter() {
                seq.serialize_element(&port)?;
            }
            seq.end()
        } else {
            serializer.serialize_none()
        }
    }        
}

impl Serialize for CellView {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {    
        let len_interface = self.interface.len();
        let len_contents = self.contents.len();
        let local_size = 2 + len_interface + len_contents;
        let mut seq = serializer.serialize_seq(Some(local_size))?;
        let viewtype = ("viewtype", "NETLIST");

        seq.serialize_element("view")?;
        seq.serialize_element(&self.name)?;
        seq.serialize_element(&viewtype)?;

        if !self.interface.is_empty() {
            seq.serialize_element(&self.interface)?;
        }

        if !self.contents.is_empty() {
            seq.serialize_element(&self.contents)?;
        }

        seq.end()
    }        
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