#[inline]
fn include_windows() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("resources/icon.ico");
    res.compile().unwrap();
}

fn main() {
    #[cfg_attr(not(debug_assetions), cfg(windows))]
    include_windows();
}
