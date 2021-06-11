use std::process::Command;
use edifier::Edif;
use expect_test::expect;

/*
#[test]
fn runs_without_arguments() {
    let mut cmd = Command::cargo_bin("ls").unwrap();
    cmd.assert().success();
}*/

#[test]
fn empty() {
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


