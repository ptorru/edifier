/*
MIT License

Copyright (c) 2021 Pedro M. Torruella N.
*/

//use std::process::Command;
//use Edif;
use edifier::ast::*;
use std::cmp::Ordering;

/*
#[test]
fn runs_without_arguments() {
    let mut cmd = Command::cargo_bin("ls").unwrap();
    cmd.assert().success();
}*/

fn match_check(incoming: String) -> i32 {
    let mut counter: i32 = 0;
    for c in incoming.chars() {
        if c == '(' {
            counter += 1;
        } else if c == ')' {
            counter -= 1;
        }
    }
    match counter.cmp(&0) {
        Ordering::Greater => println!("ERROR: Too many left parentheses."),
        Ordering::Less => println!("ERROR: Too many right parentheses."),
        Ordering::Equal => (),
    }
    counter
}

// Test 1: we should get a minimal edif with no elements
#[test]
fn empty_edif() {
    let ed = Edif {
        name: "ed".to_string(),
        elements: Vec::new(),
    };
    let actual = serde_sexpr::to_string(&ed).unwrap();

    assert_eq!(
        actual,
        "(edif ed (edifversion 2 0 0) (edifLevel 0) (keywordmap (keywordlevel 0)))"
    );
    assert_eq!(match_check(actual), 0);
}

// Test 2: single library element
#[test]
fn edif_lib() {
    let lib = Library {
        name: "mylib".to_string(),
        elements: Vec::new(),
    };
    let libelem = EdifElements::Library(lib);
    let ed = Edif {
        name: "ed2".to_string(),
        elements: vec![libelem],
    };
    let actual = serde_sexpr::to_string(&ed).unwrap();

    assert_eq!(actual,
        "(edif ed2 (edifversion 2 0 0) (edifLevel 0) (keywordmap (keywordlevel 0)) ((Library mylib)))" );
    assert_eq!(match_check(actual), 0);
}

// Test 3: cell with no elements
#[test]
fn cell_empty() {
    let mycell = Cell {
        name: "mycell".to_string(),
        views: Vec::new(),
    };
    let actual = serde_sexpr::to_string(&mycell).unwrap();
    assert_eq!(actual, "(cell mycell (celltype GENERIC))");
    assert_eq!(match_check(actual), 0);
}

// Test 4: cell view with no elements
#[test]
fn cellview_empty() {
    let myview = CellView {
        name: "myview".to_string(),
        interface: CellInterface(Vec::new()),
        contents: CellContents(Vec::new()),
    };
    let actual = serde_sexpr::to_string(&myview).unwrap();
    assert_eq!(actual, "(view myview (viewtype NETLIST))");
    assert_eq!(match_check(actual), 0);
}

// Test 5: interface with no elements
#[test]
fn interface_empty() {
    let myinterface = CellInterface(Vec::new());
    let actual = serde_sexpr::to_string(&myinterface).unwrap();
    assert_eq!(actual, "()");
    assert_eq!(match_check(actual), 0);
}

// Test 6: interface with 2 elements
#[test]
fn interface_some() {
    let dirin = PortElements::Direction(PortDirection::Input);
    let porta = InterfacePort {
        name: "a".to_string(),
        element: dirin,
    };
    let dirin = PortElements::Direction(PortDirection::Input);
    let portb = InterfacePort {
        name: "b".to_string(),
        element: dirin,
    };
    let myinterface = CellInterface(vec![porta, portb]);
    let actual = serde_sexpr::to_string(&myinterface).unwrap();
    assert_eq!(
        actual,
        "(interface (port a (direction INPUT)) (port b (direction INPUT)))"
    );
    assert_eq!(match_check(actual), 0);
}

// Test 7: contents  empty
#[test]
fn contents_empty() {
    let mycontent = CellContents(Vec::new());
    let actual = serde_sexpr::to_string(&mycontent).unwrap();
    assert_eq!(actual, "()");
    assert_eq!(match_check(actual), 0);
}

//test 8: content instance with no properties
#[test]
fn contents_instance_simple() {
    let myinstance = ContentInstance {
        name: "lut4".to_string(),
        viewref: "myview".to_string(),
        cellref: "mycellref".to_string(),
        libraryref: "mylibref".to_string(),
        properties: PropertyList(Vec::new()),
    };
    let actual = serde_sexpr::to_string(&myinstance).unwrap();
    assert_eq!(
        actual,
        "(instance lut4 (viewref myview (cellref mycellref (libraryref mylibref))))"
    );
    assert_eq!(match_check(actual), 0);
}

//test 8.1: content instance with properties
#[test]
fn contents_instance_props() {
    let props = PropertyList(vec![
        Property {
            name: "adjustability".to_string(),
            property: PropertyValue::Integer(11),
        },
        Property {
            name: "usability".to_string(),
            property: PropertyValue::String("very_high_please".to_string()),
        },
    ]);
    let myinstance = ContentInstance {
        name: "dsp1".to_string(),
        viewref: "myview".to_string(),
        cellref: "mycellref".to_string(),
        libraryref: "mylibref".to_string(),
        properties: props,
    };
    let actual = serde_sexpr::to_string(&myinstance).unwrap();
    assert_eq!(
        actual,
        r#"(instance dsp1 (viewref myview (cellref mycellref (libraryref mylibref))) (property adjustability (integer 11)) (property usability (string "very_high_please")))"#
    );
    assert_eq!(match_check(actual), 0);
}

//test 9: content net
#[test]
fn net_empty() {
    let myinstance = ContentNet::new("y");
    let actual = serde_sexpr::to_string(&myinstance).unwrap();
    assert_eq!(actual, "(net y)");
    assert_eq!(match_check(actual), 0);
}

//test 10: port reference
#[test]
fn portref() {
    let myport = PortRef {
        token: PortRefToken::Name("y".to_string()),
        instanceref: "myinst".to_string(),
    };
    let actual = serde_sexpr::to_string(&myport).unwrap();
    assert_eq!(actual, "(portref y (instanceref myinst))");
    assert_eq!(match_check(actual), 0);
}

//test 11: multiple port reference
#[test]
fn multi_portref() {
    let myport1 = PortRef {
        token: PortRefToken::Name("y".to_string()),
        instanceref: "myinst".to_string(),
    };
    let myport2 = PortRef {
        token: PortRefToken::Name("x".to_string()),
        instanceref: "".to_string(),
    };
    let my_list = PortList(vec![myport1, myport2]);
    let myinstance = ContentNet {
        name: "y".to_string(),
        portlist: my_list,
    };

    let actual = serde_sexpr::to_string(&myinstance).unwrap();
    assert_eq!(
        actual,
        "(net y (joined (portref y (instanceref myinst)) (portref x)))"
    );
    assert_eq!(match_check(actual), 0);
}

// Test 12: contents with something inside.
#[test]
fn contents_elements() {
    let mut mycontent = CellContents(Vec::new());
    mycontent.push(ContentElement::Net(ContentNet::new("a")));
    mycontent.push(ContentElement::Net(ContentNet::new("b")));

    let actual = serde_sexpr::to_string(&mycontent).unwrap();
    assert_eq!(actual, "(contents (net a) (net b))");
    assert_eq!(match_check(actual), 0);
}

// Test 13: property values
#[test]
fn property_values() {
    let mypropint = PropertyValue::Integer(42);
    let actual = serde_sexpr::to_string(&mypropint).unwrap();
    assert_eq!(actual, "(integer 42)");
    assert_eq!(match_check(actual), 0);

    let mypropstr = PropertyValue::String("64'h00AA00FF33CC0F00".to_string());
    let actual = serde_sexpr::to_string(&mypropstr).unwrap();
    assert_eq!(actual, r#"(string "64'h00AA00FF33CC0F00")"#);
    assert_eq!(match_check(actual), 0);
}

// Test 14: property
#[test]
fn property_complete() {
    let mypropval = PropertyValue::String(
        "256'h0000000000000000000000000000000000000000000000000000000000000000".to_string(),
    );
    let myprop = Property {
        name: "INITP_01".to_string(),
        property: mypropval,
    };
    let actual = serde_sexpr::to_string(&myprop).unwrap();
    assert_eq!(
        actual,
        r#"(property INITP_01 (string "256'h0000000000000000000000000000000000000000000000000000000000000000"))"#
    );
    assert_eq!(match_check(actual), 0);
}
