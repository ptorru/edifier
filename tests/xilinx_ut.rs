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
use edifier::ast::*;

pub fn new_lut2() -> Cell {
    let porto = InterfacePort::new_output("O");

    let porti0 = InterfacePort::new_input("I0");

    let porti1 = InterfacePort::new_input("I1");

    let interface = CellInterface(vec![porto, porti0, porti1]);

    let cellview = CellView {
        name: "netlist".to_string(),
        interface,
        contents: CellContents(Vec::new()),
    };
    Cell {
        name: "LUT2".to_string(),
        views: CellViews(vec![cellview]),
    }
}

pub fn lut2_prop_ini<S>(val: S) -> Property
where
    S: AsRef<str>,
{
    Property {
        name: "INIT".to_string(),
        property: PropertyValue::from(val.as_ref().to_string()),
    }
}

pub fn lut2_prop_box<S>(val: S) -> Property
where
    S: AsRef<str>,
{
    Property {
        name: "BOX_TYPE".to_string(),
        property: PropertyValue::from(val.as_ref().to_string()),
    }
}

pub fn lut2_prop_loc<S>(val: S) -> Property
where
    S: AsRef<str>,
{
    Property {
        name: "LOC".to_string(),
        property: PropertyValue::from(val.as_ref().to_string()),
    }
}

pub fn lut2_prop_bel<S>(val: S) -> Property
where
    S: AsRef<str>,
{
    Property {
        name: "BEL".to_string(),
        property: PropertyValue::from(val.as_ref().to_string()),
    }
}

pub fn carry8() -> Cell {
    let mut interface = CellInterface(Vec::new());

    interface.push(InterfacePort::new_input("CI"));
    interface.push(InterfacePort::new_input("CI_TOP"));
    interface.push(InterfacePort::new(
        PortToken::new_array("CO", "CO[7:0]", 8),
        PortDirection::Output,
    ));
    interface.push(InterfacePort::new(
        PortToken::new_array("O", "O[7:0]", 8),
        PortDirection::Output,
    ));
    interface.push(InterfacePort::new(
        PortToken::new_array("DI", "DI[7:0]", 8),
        PortDirection::Input,
    ));
    interface.push(InterfacePort::new(
        PortToken::new_array("S", "S[7:0]", 8),
        PortDirection::Input,
    ));

    let cellview = CellView {
        name: "netlist".to_string(),
        interface,
        contents: CellContents(Vec::new()),
    };
    Cell {
        name: "CARRY8".to_string(),
        views: CellViews(vec![cellview]),
    }
}

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
    proplist.push(lut2_prop_ini("4'h6"));
    proplist.push(lut2_prop_box("PRIMITIVE"));
    proplist.push(lut2_prop_loc("SLICE_X0Y0"));
    proplist.push(lut2_prop_bel("A6LUT"));

    let contents = ContentInstance {
        token: StringToken::new("i0"),
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

// Test 3: we should get an instance with properties
//         for a placed lut2 element, being renamed
#[test]
fn lut2_instance_renamed() {
    let mut proplist = PropertyList(Vec::new());
    proplist.push(lut2_prop_ini("4'h6"));
    proplist.push(lut2_prop_box("PRIMITIVE"));
    proplist.push(lut2_prop_loc("SLICE_X0Y0"));
    proplist.push(lut2_prop_bel("A6LUT"));

    let contents = ContentInstance {
        token: StringToken::new_renamed(
            "address_loop_2__output_data_pc_vector_mux_lut",
            r#"address_loop[2].output_data.pc_vector_mux_lut"#,
        ),
        viewref: "netlist".to_string(),
        cellref: "LUT2".to_string(),
        libraryref: "hdi_primitives".to_string(),
        properties: proplist,
    };

    let actual = serde_sexpr::to_string(&contents).unwrap();

    assert_eq!(
        actual,
        r#"(instance (rename address_loop_2__output_data_pc_vector_mux_lut "address_loop[2].output_data.pc_vector_mux_lut") (viewref netlist (cellref LUT2 (libraryref hdi_primitives))) (property INIT (string "4'h6")) (property BOX_TYPE (string "PRIMITIVE")) (property LOC (string "SLICE_X0Y0")) (property BEL (string "A6LUT")))"#
    );
}

// Test 4: Testing arrays
#[test]
fn simple_array() {
    let ed = carry8();

    let actual = serde_sexpr::to_string(&ed).unwrap();

    assert_eq!(
        actual,
        r#"(cell CARRY8 (celltype GENERIC) (view netlist (viewtype NETLIST) (interface (port CI (direction INPUT)) (port CI_TOP (direction INPUT)) (port (array (rename CO "CO[7:0]") 8) (direction OUTPUT)) (port (array (rename O "O[7:0]") 8) (direction OUTPUT)) (port (array (rename DI "DI[7:0]") 8) (direction INPUT)) (port (array (rename S "S[7:0]") 8) (direction INPUT)))))"#
    );
    //assert_eq!(match_check(actual), 0);
}
