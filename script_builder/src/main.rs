use std::{error::Error, fs};

use script_builder::parse_script;


fn main() -> Result<(), Box<dyn Error>> {
    let mut script = fs::read_to_string("input/script.dailog")?;

    let json = match parse_script(&mut script) {
        Ok(val) => val,
        Err(e) => { 
            println!("{e}");
            return Err(e); 
        },
    };

    fs::write("output/script.json", format!("{:#}", json.result))?;

    Ok(())
}
