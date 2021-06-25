/*
MIT License

Copyright (c) 2021 Pedro M. Torruella N.
*/
use derive_more::{Deref, DerefMut};

// TODO: add member
#[derive(Debug)]
pub struct PortRef {
    pub name: String,
    pub instanceref: String,
}

#[derive(Debug, Deref, DerefMut)]
pub struct PortList(pub Vec<PortRef>);

#[derive(Debug)]
// TODO: add support for rename
pub struct ContentNet {
    pub name: String,
    pub portlist: PortList,
}

#[derive(Debug)]
// TODO: add support for rename
pub struct ContentInstance {
    pub name: String,
    pub viewref: String,
    pub cellref: String,
    pub libraryref: String,
}

#[derive(Debug)]
pub enum ContentElement {
    Instance,
    Net,
}

#[derive(Debug, Deref, DerefMut)]
pub struct CellContents(pub Vec<ContentElement>);

#[derive(Debug)]
pub enum PortDirection {
    Input,
    Output,
}

#[derive(Debug)]
pub enum PortElements {
    Direction(PortDirection),
    //Array(PortArray)
}

#[derive(Debug)]
pub struct InterfacePort {
    pub name: String,
    pub element: PortElements,
}

#[derive(Debug, Deref, DerefMut)]
pub struct CellInterface(pub Vec<InterfacePort>);

#[derive(Debug)]
pub struct CellView {
    pub name: String,
    pub interface: CellInterface,
    pub contents: CellContents,
}

#[derive(Debug)]
pub struct Cell {
    pub name: String,
    pub views: Vec<CellView>,
}

#[derive(Debug)]
pub struct Library {
    pub name: String,
    pub elements: Vec<Cell>,
}

#[derive(Debug)]
pub enum EdifElements {
    Library(Library),
    //Design(String)
}

#[derive(Debug)]
pub struct Edif {
    pub name: String,
    pub elements: Vec<EdifElements>,
}
