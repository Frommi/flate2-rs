use std::process::Command;
use std::path::Path;
use std::ffi::OsString;
use std::os::unix::ffi::OsStringExt;

fn main() {
    Command::new("./miniz_oxide/build.sh").status().unwrap();
    let cur_dir = {
        let cur_dir = Command::new("pwd").output().unwrap().stdout;
        let n = cur_dir.len();
        // Strip the newline.
        Vec::from(&cur_dir[..n - 1])
    };
    let out_dir = Path::new(&OsString::from_vec(cur_dir)).join("miniz_oxide");
    println!("cargo:rustc-link-search=native={}", out_dir.to_str().unwrap());
}
