use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct ReplacementOptions {
    backup: bool,
    restore: bool,
}

#[derive(Deserialize, Debug)]
struct Application {
    name: String,
    dir: Vec<String>,
    replacements: Vec<Vec<String>>,
    options: ReplacementOptions,
}

fn main() -> Result<(), &'static str> {
    println!("Hello, world!");
    let file_path = std::env::args().nth(1).expect("No file given");
    println!("File: {file_path}");

    let file = File::open(file_path).expect("Can't open file");
    let reader = BufReader::new(file);

    let app: Application = serde_json::from_reader(reader).expect("Couldn't parse file");
    println!("{:#?}", app);

    if app.options.backup == false || app.options.restore == false {
        return Err("Disabling backup and restore options not implemented");
    }

    let binding = app.dir.join("/");
    let dir_path = Path::new(&binding);
    println!("{}", dir_path.display());

    for replacement in app.replacements {
        let origin = dir_path.join(replacement[1].clone());
        let new_file = dir_path.join(replacement[0].clone());
        let backup = origin.with_file_name(format!("{}.tuisto.bak", replacement[1].clone()));

        // Figure out if we're applying, or unapplying
        if backup.exists() {
            println!("Unapplying {}", app.name);
            std::fs::copy(backup.clone(), origin.clone()).expect("Couldn't unapply copy");
            std::fs::remove_file(backup.clone()).expect("Couldn't remove backup");
        } else {
            println!("Applying {}", app.name);
            std::fs::copy(origin.clone(), backup.clone()).expect("Couldn't apply backup");
            std::fs::copy(new_file.clone(), origin.clone()).expect("Couldn't apply replacement");
        }
    }

    Ok(())
}
