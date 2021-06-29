/*
Copyright 2021 Pedro M. Torruella N.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/
use derive_more::{Deref, DerefMut, From};

#[derive(Debug)]
pub struct Rename {
    pub from: String,
    pub to: String,
}

#[derive(Debug, From)]
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

#[derive(Debug, From)]
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

#[derive(Debug, From)]
pub enum StringToken {
    Name(String),
    Rename(Rename),
}

#[derive(Debug)]
// TODO: add support for rename
pub struct ContentNet {
    pub token: StringToken,
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

#[derive(Debug, From)]
pub enum ContentElement {
    Instance(ContentInstance),
    Net(ContentNet),
}

#[derive(Debug, Deref, DerefMut)]
pub struct CellContents(pub Vec<ContentElement>);

#[derive(Debug, From)]
pub enum PortDirection {
    Input,
    Output,
}

#[derive(Debug, From)]
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

#[derive(Debug, Deref, DerefMut)]
pub struct CellViews(pub Vec<CellView>);

#[derive(Debug)]
pub struct Cell {
    pub name: String,
    pub views: CellViews,
}

#[derive(Debug)]
pub struct Library {
    pub name: String,
    pub elements: Vec<Cell>,
}

#[derive(Debug, From)]
pub enum EdifElements {
    Library(Library),
    //Design(String)
}

#[derive(Debug)]
pub struct Edif {
    pub name: String,
    pub elements: Vec<EdifElements>,
}
