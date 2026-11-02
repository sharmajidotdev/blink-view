// fn main() {
//     println!("Hello, world!");
// }

use walkdir::WalkDir;

fn main() {
    for entry in WalkDir::new(".") {
        match entry {
            Ok(e) => println!("{}", e.path().display()),
            Err(err) => eprintln!("Error: {}", err),
        }
    }
}
