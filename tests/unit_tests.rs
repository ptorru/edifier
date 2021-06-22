//use std::process::Command;
//use edifier::Edif;


/*
#[test]
fn runs_without_arguments() {
    let mut cmd = Command::cargo_bin("ls").unwrap();
    cmd.assert().success();
}*/

fn match_check(incoming: String) -> i32{
    let mut counter: i32 = 0;
    for c in incoming.chars() { 
        if c == '(' {
            counter = counter + 1;
        }
        else if c == ')' {
            counter = counter - 1;
        }
    }
    if counter > 0 {
        println!("ERROR: Too many left parentheses.");
    }
    else if counter < 0 {
        println!("ERROR: Too many right parentheses.")
    }
    counter
}


// Test 1: we should get a minimal edif with no elements
#[test]
fn empty_edif() {
    let ed = edifier::Edif {
        name: "ed".to_string(),
        elements: Vec::new(),
    };
    let actual = serde_sexpr::to_string(&ed).unwrap();

    assert_eq!(actual, 
        "(edif ed (edifversion 2 0 0) (edifLevel 0) (keywordmap (keywordlevel 0)))");
    assert_eq!(match_check(actual), 0);
}

// Test 2: single library element
#[test]
fn edif_lib() {
    let lib = edifier::Library {
        name: "mylib".to_string(),
        elements: Vec::new(),
    };
    let libelem = edifier::EdifElements::Library(lib);
    let ed = edifier::Edif {
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
    let mycell = edifier::Cell {
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
    let myview = edifier::CellView {
        name: "myview".to_string(),
        interface: edifier::CellInterface(Vec::new()) ,
        contents: edifier::CellContents(Vec::new()),
    };
    let actual = serde_sexpr::to_string(&myview).unwrap();
    assert_eq!(actual, "(view myview (viewtype NETLIST))");
    assert_eq!(match_check(actual), 0);
}

// Test 5: interface with no elements
#[test]
fn interface_empty() {
    let myinterface = edifier::CellInterface(Vec::new());
    let actual = serde_sexpr::to_string(&myinterface).unwrap();
    assert_eq!(actual, "()");
    assert_eq!(match_check(actual), 0);
}

// Test 6: interface with 2 elements
#[test]
fn interface_some() {
    let dirin = edifier::PortElements::Direction(edifier::PortDirection::Input);
    let porta = edifier::InterfacePort{
            name: "a".to_string(), 
            element: dirin
        };
    let dirin = edifier::PortElements::Direction(edifier::PortDirection::Input);
    let portb = edifier::InterfacePort{
            name: "b".to_string(), 
            element:  dirin
        };
    let myinterface = edifier::CellInterface(vec![porta, portb]);
    let actual = serde_sexpr::to_string(&myinterface).unwrap();
    assert_eq!(actual, "(interface (port a (direction INPUT)) (port b (direction INPUT)))");
    assert_eq!(match_check(actual), 0);
}

// Test 7: contents  empty
#[test]
fn contents_empty() {
    let mycontent = edifier::CellContents(Vec::new());
    let actual = serde_sexpr::to_string(&mycontent).unwrap();
    assert_eq!(actual, "()");
    assert_eq!(match_check(actual), 0);
}

//test 8: content instance
#[test]
fn contents_instance() {
    let myinstance = edifier::ContentInstance { name: "lut4".to_string(),
                                               viewref: "myview".to_string(), 
                                               cellref: "mycellref".to_string(), 
                                               libraryref: "mylibref".to_string(),
                                            };
    let actual = serde_sexpr::to_string(&myinstance).unwrap();
    assert_eq!(actual, "(instance lut4 (viewref myview (cellref mycellref (libraryref mylibref))))");
    assert_eq!(match_check(actual), 0);
}

//test 9: content net
#[test]
fn net_empty() {
    let my_list = edifier::PortList(Vec::new());
    let myinstance = edifier::ContentNet { name: "y".to_string(),
                                           portlist: my_list, 
                                         };
    let actual = serde_sexpr::to_string(&myinstance).unwrap();
    assert_eq!(actual, "(net y)");
    assert_eq!(match_check(actual), 0);
}

//test 10: port referemce
#[test]
fn portref() {
    let myport = edifier::PortRef { name: "y".to_string(),
                                        instanceref: "myinst".to_string(), 
                                        };
    let actual = serde_sexpr::to_string(&myport).unwrap();
    assert_eq!(actual, "(portref y (instanceref myinst))");
    assert_eq!(match_check(actual), 0);
}

//test 11: multiple port reference
#[test]
fn multi_portref() {
    let myport1 = edifier::PortRef { name: "y".to_string(),
                                        instanceref: "myinst".to_string(), 
                                        };
    let myport2 = edifier::PortRef { name: "x".to_string(),
                                        instanceref: "".to_string(), 
                                        };
    let my_list = edifier::PortList(vec![myport1, myport2]);
    let myinstance = edifier::ContentNet { name: "y".to_string(),
    portlist: my_list, 
    };

    let actual = serde_sexpr::to_string(&myinstance).unwrap();
    assert_eq!(actual, "(net y (joined (portref y (instanceref myinst)) (portref x )))");
    assert_eq!(match_check(actual), 0);
}
