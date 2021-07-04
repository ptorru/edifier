use edifier::ast::*;
use edifier::string_helpers::add_new_lines;

fn main() {
    let top_name = "inverter".to_string();
    let work_name = "work".to_string();

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

    let yinst0_name = "y_INST_0";

    let yinst0 = ContentElement::from(ContentInstance {
        token: StringToken::new(yinst0_name.clone()),
        viewref: "myview".to_string(),
        cellref: CellRef::new("LUT2", "hdi_primitives"),
        properties: PropertyList(vec![Property::new_string("INIT", "4'h8")]),
    });

    let neta_name = "a";
    let neta = ContentElement::from(ContentNet::new_with_ports(
        neta_name.clone(),
        PortList(vec![
            PortRef::new_with_ref("I0", InstanceRef::new(yinst0_name.clone())),
            PortRef::new(neta_name.clone()),
        ]),
    ));

    let netb_name = "b";
    let netb = ContentElement::from(ContentNet::new_with_ports(
        netb_name.clone(),
        PortList(vec![
            PortRef::new_with_ref("I1", InstanceRef::new(yinst0_name.clone())),
            PortRef::new(netb_name.clone()),
        ]),
    ));

    let nety_name = "y";
    let nety = ContentElement::from(ContentNet::new_with_ports(
        nety_name.clone(),
        PortList(vec![
            PortRef::new_with_ref("O", InstanceRef::new(yinst0_name.clone())),
            PortRef::new(nety_name.clone()),
        ]),
    ));

    let inv = Cell {
        name: top_name.clone(),
        views: CellViews(vec![CellView {
            name: top_name.clone(),
            interface: CellInterface(vec![
                InterfacePort::new_input("neta_name".clone()),
                InterfacePort::new_input("netb_name".clone()),
                InterfacePort::new_output("nety_name".clone()),
            ]),
            contents: CellContents(vec![yinst0, neta, netb, nety]),
        }]),
    };

    let lib_work = Library {
        name: work_name.clone(),
        elements: Cells::from(vec![inv]),
    };

    let design_inv = Design::new_with_prop(
        top_name.clone(),
        CellRef::new(top_name.clone(), work_name.clone()),
        PropertyList(vec![Property::new_string("part", "xczu3eg-sbva484-1-e")]),
    );

    let libp = EdifElement::from(lib_prims);
    let libw = EdifElement::from(lib_work);
    let desi = EdifElement::from(design_inv);

    let edif = Edif {
        name: top_name.clone(),
        elements: EdifElements::from(vec![libp, libw, desi]),
    };

    let edif_string = add_new_lines(serde_sexpr::to_string(&edif).unwrap(), 4, true);

    println!("{}", edif_string);
}
