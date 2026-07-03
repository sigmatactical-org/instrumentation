fn main() {
    println!("cargo::rustc-check-cfg=cfg(co_pilot_embedded)");
    println!("cargo::rustc-check-cfg=cfg(feature, values(\"virt-panel\"))");
    slint_build::compile("ui/app.slint").unwrap();
}
