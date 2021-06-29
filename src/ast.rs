/*
MIT License

Copyright (c) 2021 Pedro M. Torruella N.
*/
use derive_more::{Deref, DerefMut};

#[derive(Debug)]
pub enum PropertyValue {
    Integer(i32),
    String(String),
}

#[derive(Debug)]
pub struct Property {
    pub name: String,
    pub property: PropertyValue,
}

#[derive(Debug, Deref, DerefMut)]
pub struct PropertyList(pub Vec<Property>);

#[derive(Debug)]
pub struct PortMember {
    pub name: String,
    pub index: u32,
}

#[derive(Debug)]
pub enum PortRefToken {
    Name(String),
    Member(PortMember),
}

#[derive(Debug)]
pub struct PortRef {
    pub token: PortRefToken,
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
    pub properties: PropertyList,
}

#[derive(Debug)]
pub enum ContentElement {
    Instance(ContentInstance),
    Net(ContentNet),
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
