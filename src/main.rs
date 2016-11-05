use std::fs;

fn main() {
    match fs::create_dir("grabbed_photos") {
        Ok(res) => res,
        Err(e) => println!("Something went wrong: {}", e),
    }
    fs::copy("photos/1.jpg", "photos/1.jpg");
}
