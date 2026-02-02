// build.rs
fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("my_icon.ico"); // Replace with your icon filename
        res.compile().unwrap();
    }
}
