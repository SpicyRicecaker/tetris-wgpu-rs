#[inline]
fn _include_windows() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("resources/icon.ico");
    res.compile().unwrap();
}

fn main() {
    #[cfg(windows)]
    #[cfg(not(debug_assertions))]
    _include_windows();
}
