use std::process::Command;

fn main() {
    Command::new("./miniz_oxide/build.sh").status().unwrap();
    let out_dir = "miniz-sys/miniz_oxide";
    println!("cargo:rustc-link-search=native={}", out_dir);
}
