use edifier::ast::*;
use edifier::string_helpers::add_new_lines;

fn main() {
    let elems = Cells::from(vec![
        Cell {
            name: "LUT2".to_string(),
            views: CellViews(vec![CellView {
                name: "netlist".to_string(),
                interface: CellInterface(vec![
                    InterfacePort::new_output("O"),
                    InterfacePort::new_input("I0"),
                    InterfacePort::new_input("I1"),
                ]),
                contents: CellContents(Vec::new()),
            }]),
        },
        Cell {
            name: "INV".to_string(),
            views: CellViews(vec![CellView {
                name: "netlist".to_string(),
                interface: CellInterface(vec![
                    InterfacePort::new_input("I"),
                    InterfacePort::new_output("O"),
                ]),
                contents: CellContents(Vec::new()),
            }]),
        },
    ]);

    let lib_prims = Library {
        name: "hdi_primitives".to_string(),
        elements: elems,
    };

    let yinst0 = ContentElement::from(ContentInstance {
        token: StringToken::new("y_INST_0"),
        viewref: "myview".to_string(),
        cellref: CellRef::new("LUT2", "hdi_primitives"),
        properties: PropertyList(vec![Property::new_string("INIT", "4'h8")]),
    });

    let neta = ContentElement::from(ContentNet::new_with_ports(
        "a",
        PortList(vec![PortRef::new("a")]),
    ));

    let inv = Cell {
        name: "inverter".to_string(),
        views: CellViews(vec![CellView {
            name: "inverter".to_string(),
            interface: CellInterface(vec![
                InterfacePort::new_input("a"),
                InterfacePort::new_input("b"),
                InterfacePort::new_output("y"),
            ]),
            contents: CellContents(vec![yinst0, neta]),
        }]),
    };

    let lib_work = Library {
        name: "work".to_string(),
        elements: Cells::from(vec![inv]),
    };

    let libp = EdifElement::from(lib_prims);
    let libw = EdifElement::from(lib_work);

    let edif = Edif {
        name: "inverter".to_string(),
        elements: EdifElements::from(vec![libp, libw]),
    };

    let edif_string = add_new_lines(serde_sexpr::to_string(&edif).unwrap(), 4, true);

    println!("{}", edif_string);
}
