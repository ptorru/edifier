use edifier::ast::*;
use edifier::xilinx::*;

// Test 1: we should get a lut2 element
#[test]
fn lut2() {
    let ed = new_lut2();

    let actual = serde_sexpr::to_string(&ed).unwrap();

    assert_eq!(
        actual,
        "(cell LUT2 (celltype GENERIC) (view netlist (viewtype NETLIST) (interface (port O (direction OUTPUT)) (port I0 (direction INPUT)) (port I1 (direction INPUT)))))"
    );
    //assert_eq!(match_check(actual), 0);
}

// Test 2: we should get an instance with properties
//         for a placed lut2 element
#[test]
fn lut2_instance() {
    let mut proplist = PropertyList(Vec::new());
    proplist.push(lut2_prop_ini("4'h6".to_string()));
    proplist.push(lut2_prop_box("PRIMITIVE".to_string()));
    proplist.push(lut2_prop_loc("SLICE_X0Y0".to_string()));
    proplist.push(lut2_prop_bel("A6LUT".to_string()));

    let contents = ContentInstance {
        name: "i0".to_string(),
        viewref: "netlist".to_string(),
        cellref: "LUT2".to_string(),
        libraryref: "hdi_primitives".to_string(),
        properties: proplist,
    };

    let actual = serde_sexpr::to_string(&contents).unwrap();

    assert_eq!(
        actual,
        r#"(instance i0 (viewref netlist (cellref LUT2 (libraryref hdi_primitives))) (property INIT (string "4'h6")) (property BOX_TYPE (string "PRIMITIVE")) (property LOC (string "SLICE_X0Y0")) (property BEL (string "A6LUT")))"#
    );
}
