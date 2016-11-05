use std::fs;

fn main() {
    match fs::create_dir("grabbed_photos") {
        Ok(res) => res,
        Err(e) => println!("Something went wrong during directory creation: {:?}", e),
    }
    match fs::copy("photos/1.jpg", "grabbed_photos/1.jpg") {
        Ok(res) => println!("Results was: {}", res),
        Err(e) => println!("Something went wrong during file copy: {:?}", e),
    }
}
