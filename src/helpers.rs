/*
MIT License

Copyright (c) 2021 Pedro M. Torruella N.
*/

use crate::ast::*;

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
            name: name.as_ref().to_string(),
            portlist: PortList(Vec::new()),
        }
    }
}
