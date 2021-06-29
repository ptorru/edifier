use crate::ast::*;


pub fn new_lut2() -> Cell
{

    let diro = PortElements::Direction(PortDirection::Output);
    let porto = InterfacePort {
        name: "O".to_string(),
        element: diro,
    };

    let dirin = PortElements::Direction(PortDirection::Input);
    let porti0 = InterfacePort {
        name: "I0".to_string(),
        element: dirin,
    };

    let dirin = PortElements::Direction(PortDirection::Input);
    let porti1 = InterfacePort {
        name: "I1".to_string(),
        element: dirin,
    };

    let interface = CellInterface(vec![porto, porti0, porti1]);

    let cellview = CellView {
        name: "netlist".to_string(),
        interface,
        contents: CellContents(Vec::new()),
    };
    Cell {
        name: "LUT2".to_string(),
        views: CellViews(vec![cellview])
    }
}