use std::process::Command;
use edifier::Edif;
use expect_test::expect;

/*
#[test]
fn runs_without_arguments() {
    let mut cmd = Command::cargo_bin("ls").unwrap();
    cmd.assert().success();
}*/

// Test 1: we should get a minimal edif with no elements
#[test]
fn empty_edif() {
    let ed = edifier::Edif {
        design_name: "ed".to_string(),
        libraries: Vec::new(),
        comments: Vec::new(),
        designs: Vec::new(),
    };
    let actual = serde_sexpr::to_string(&ed).unwrap();
    let expected = expect![["(edif ed (edifversion 2 0 0) (edifLevel 0) (keywordmap (keywordlevel 0)))"]];
    expected.assert_debug_eq(&actual);
}

// Test 2: single library element
#[test]
fn single_lib() {
    let lib = edifier::Library {
        name: "mylib".to_string(),
        elem: "(cell LUT2 (celltype GENERIC))".to_string()
    };
    let ed = edifier::Edif {
        design_name: "ed".to_string(),
        libraries: vec![lib],
        comments: Vec::new(),
        designs: Vec::new(),
    };
    let actual = serde_sexpr::to_string(&ed).unwrap();
    let expected = expect![["(edif ed (edifversion 2 0 0) (edifLevel 0) (keywordmap (keywordlevel 0)) ((Library mylib (cell LUT2 (celltype GENERIC)))))"]];
    expected.assert_debug_eq(&actual);
}
