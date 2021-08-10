use std::path::Path;
use std::fs::File;
use std::io::Write;
use platforms::xilinx::inverter;
use edifier::string_helpers::add_new_lines;


static DOFILE: bool = true;


fn main() {
    let edif = inverter();

    let serialized = serde_sexpr::to_string(&edif).unwrap();
    let edif_string = add_new_lines(serialized, 5, true);

    if DOFILE {
        let path = Path::new("xilinx_inverter.edf");
        let display = path.display();

        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        match file.write_all(edif_string.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }
    } else {
        println!("{}", edif_string);
    };
}
