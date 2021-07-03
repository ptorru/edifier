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
use crate::ast::*;

impl Rename {
    pub fn new<S>(from: S, to: S) -> Self
    where
        S: AsRef<str>,
    {
        Rename {
            from: from.as_ref().to_string(),
            to: to.as_ref().to_string(),
        }
    }
}

impl GenericRef {
    pub fn new<S>(name: S, reference: S) -> Self
    where
        S: AsRef<str>,
    {
        GenericRef {
            name: name.as_ref().to_string(),
            reference: reference.as_ref().to_string(),
        }
    }
}

impl LibraryRef {
    pub fn new<S>(reference: S) -> Self
    where
        S: AsRef<str>,
    {
        LibraryRef::from(GenericRef::new("libraryref", reference.as_ref()))
    }
}

impl InstanceRef {
    pub fn new<S>(reference: S) -> Self
    where
        S: AsRef<str>,
    {
        InstanceRef::from(GenericRef::new("instanceref", reference.as_ref()))
    }
}

impl CellRef {
    pub fn new<S>(name: S, libref: S) -> Self
    where
        S: AsRef<str>,
    {
        CellRef {
            name: name.as_ref().to_string(),
            libraryref: LibraryRef::new(libref),
        }
    }
}

impl Property {
    // TODO: is there a way to have a single def of new for multiple types?
    pub fn new_integer<S>(name: S, val: i32) -> Self
    where
        S: AsRef<str>,
    {
        Property {
            name: name.as_ref().to_string(),
            property: PropertyValue::from(val),
        }
    }

    pub fn new_string<S>(name: S, val: S) -> Self
    where
        S: AsRef<str>,
    {
        Property {
            name: name.as_ref().to_string(),
            property: PropertyValue::from(val.as_ref().to_string()),
        }
    }
}

impl PortRefToken {
    pub fn new<S>(name: S) -> Self
    where
        S: AsRef<str>,
    {
        PortRefToken::Name(name.as_ref().to_string())
    }
}

impl PortRef {
    pub fn new<S>(name: S) -> Self
    where
        S: AsRef<str>,
    {
        PortRef {
            token: PortRefToken::new(name),
            instanceref: "".to_string(),
        }
    }
}

impl ContentNet {
    pub fn new<S>(name: S) -> Self
    where
        S: AsRef<str>,
    {
        ContentNet {
            token: StringToken::from(name.as_ref().to_string()),
            portlist: PortList(Vec::new()),
        }
    }

    pub fn new_remaned<S>(from: S, to: S) -> Self
    where
        S: AsRef<str>,
    {
        ContentNet {
            token: StringToken::from(Rename {
                from: from.as_ref().to_string(),
                to: to.as_ref().to_string(),
            }),
            portlist: PortList(Vec::new()),
        }
    }
}

impl StringToken {
    pub fn new<S>(name: S) -> Self
    where
        S: AsRef<str>,
    {
        StringToken::from(name.as_ref().to_string())
    }

    pub fn new_renamed<S>(from: S, to: S) -> Self
    where
        S: AsRef<str>,
    {
        StringToken::from(Rename {
            from: from.as_ref().to_string(),
            to: to.as_ref().to_string(),
        })
    }
}

impl PortArray {
    pub fn new<S>(from: S, to: S, len: i32) -> Self
    where
        S: AsRef<str>,
    {
        PortArray {
            rename: Rename::new(from, to),
            length: len,
        }
    }
}

impl PortToken {
    pub fn new<S>(name: S) -> Self
    where
        S: AsRef<str>,
    {
        PortToken::from(name.as_ref().to_string())
    }

    // TODO: How to automatically define the type of len
    //       to be the same time as PortArray.length?
    pub fn new_array<S>(from: S, to: S, len: i32) -> Self
    where
        S: AsRef<str>,
    {
        PortToken::from(PortArray::new(from, to, len))
    }
}

impl InterfacePort {
    pub fn new(porttoken: PortToken, dir: PortDirection) -> Self {
        InterfacePort {
            token: porttoken,
            direction: dir,
        }
    }

    pub fn new_input<S>(name: S) -> Self
    where
        S: AsRef<str>,
    {
        InterfacePort {
            token: PortToken::new(name),
            direction: PortDirection::Input,
        }
    }

    pub fn new_output<S>(name: S) -> Self
    where
        S: AsRef<str>,
    {
        InterfacePort {
            token: PortToken::new(name),
            direction: PortDirection::Output,
        }
    }
}

impl Design {
    pub fn new<S>(name: S, cell: CellRef) -> Self
    where
        S: AsRef<str>,
    {
        Design {
            name: name.as_ref().to_string(),
            cellref: cell,
            properties: PropertyList::from(Vec::new()),
        }
    }

    pub fn new_with_prop<S>(name: S, cell: CellRef, props: PropertyList) -> Self
    where
        S: AsRef<str>,
    {
        Design {
            name: name.as_ref().to_string(),
            cellref: cell,
            properties: props,
        }
    }
}
