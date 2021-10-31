use std::fs;
use std::path::Path;

fn main() {
    if !Path::new("./src/pb").exists() {
        match fs::create_dir("./src/pb") {
            Err(why) => println!("Failed to create directory, {:?}", why.kind()),
            Ok(_) => {}
        };
    }
    prost_build::Config::new()
        .out_dir("./src/pb")
        .compile_protos(&["abi.proto"], &["."])
        .unwrap();
}
