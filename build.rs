use std::env;
fn main() {
    let pgrx_home = env::var("PGRX_HOME").unwrap_or_else(|_| {
        let home_dir = env::var("HOME").expect("HOME environment variable not set");
        format!("{}/.pgrx", home_dir)
    });
    println!("cargo:rustc-env=PGRX_HOME={}", pgrx_home);
}