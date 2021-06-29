use edifier::xilinx::*;

// Test 1: we should get a minimal edif with no elements
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
