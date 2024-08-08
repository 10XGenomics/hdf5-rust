fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rustc-check-cfg=cfg(windows_dll)");
    if std::env::var_os("DEP_HDF5_MSVC_DLL_INDIRECTION").is_some() {
        println!("cargo::rustc-cfg=windows_dll");
    }
}
