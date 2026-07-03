fn main() {
    println!("cargo::rustc-check-cfg=cfg(co_pilot_embedded)");
    slint_build::compile("ui/app.slint").unwrap();
}
