use std::fs;
use std::path::PathBuf;

fn main() {
    // read target directory.
    // TODO: make recursive, handle errors (own function), use user-given target.
    let contents = fs::read_dir("photos").unwrap();

    // create directory for dump.
    // TODO: handle error if dir exists, use existing dir, use user-given target.
    match fs::create_dir("grabbed_photos") {
        Ok(res) => res,
        Err(e) => println!("Something went wrong during directory creation: {}", e),
    }

    // iterate over files in target dir, copy to dest dir.
    // TODO: handle errors in file_path
    for file in contents {
        let file_path = file.unwrap().path();
        let file_name = file_path.file_name().unwrap();
        println!("Copying file: {:?}", file_path);
        let mut dest_path = PathBuf::new();
        dest_path.push("grabbed_photos");
        dest_path.push(file_name);
        match fs::copy(file_path.as_path(), dest_path) {
            Ok(_) => println!("Successfully copied"),
            Err(e) => panic!("Something went wrong during copy: {}", e),
        }
    }
}
