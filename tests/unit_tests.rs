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
        libraries: Vec::new(),
        comments: Vec::new(),
        designs: Vec::new(),
    };
    let actual = serde_sexpr::to_string(&ed).unwrap();

    assert_eq!(actual, 
        "(edif ed (edifversion 2 0 0) (edifLevel 0) (keywordmap (keywordlevel 0)))");
    assert_eq!(match_check(actual), 0);
}

// Test 2: single library element
#[test]
fn single_lib() {
    let lib = edifier::Library {
        name: "mylib".to_string(),
        elem: "cell".to_string()
    };
    let ed = edifier::Edif {
        name: "ed2".to_string(),
        libraries: vec![lib],
        comments: Vec::new(),
        designs: Vec::new(),
    };
    let actual = serde_sexpr::to_string(&ed).unwrap();

    assert_eq!(actual,
        "(edif ed2 (edifversion 2 0 0) (edifLevel 0) (keywordmap (keywordlevel 0)) ((Library mylib cell)))" );
    assert_eq!(match_check(actual), 0);

}


