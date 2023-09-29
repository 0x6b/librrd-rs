use std::{env::var, path::PathBuf, process::Command};

fn main() {
    // as a starter, only macOS is supported
    if !cfg!(target_os = "macos") {
        panic!("not supported")
    }

    // skip build if `LIBRRD_SYS_SKIP_BUILD` is set, useful while developing
    if let Ok(_) = var("LIBRRD_SYS_SKIP_BUILD") {
        println!("cargo:warning=skipping build");
        return;
    }

    // run `bootstrap` script to generate `configure` script, if the script is not there
    Command::new("sh")
        .current_dir("rrdtool-1.x")
        .arg("-c")
        .arg("./bootstrap")
        .output()
        .expect("failed to execute process");

    // build rrdtool, equivalent to good old `./configure && make`
    let dst = autotools::Config::new("rrdtool-1.x")
        .reconf("-v")
        .cflag("-DSOCK_CLOEXEC=0") // for macOS 13.4.1
        .insource(true)
        .env("MAKEFLAGS", "-j8")
        .config_option("disable-docs", None)
        .config_option("disable-example", None)
        .config_option("disable-rrdcached", None)
        .config_option("disable-rrdcgi", None)
        .config_option("disable-rrd_graph", None)
        .config_option("disable-rrd_restore", None)
        .config_option("disable-rpath", None)
        .config_option("disable-libdbi", None)
        .config_option("disable-librados", None)
        .config_option("disable-libwrap", None)
        .config_option("disable-perl", None)
        .config_option("disable-ruby", None)
        .config_option("disable-lua", None)
        .config_option("disable-tcl", None)
        .config_option("disable-python", None)
        .build();

    // final `librrd.a` will located under `rrdtool-1.x/lib/` directory
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=rrd");
    println!("cargo:rustc-link-lib=c");

    // generate bindings
    let bindings = bindgen::Builder::default()
        .header(dst.join("include").join("rrd.h").to_str().unwrap())
        .layout_tests(false)
        .generate()
        .expect("unable to generate bindings");

    // basically the `build.rs` shouldn't write anything outside of `OUT_DIR`, but for now
    // let out_dir = PathBuf::from(var("OUT_DIR").unwrap());
    let out_dir = PathBuf::from(var("CARGO_MANIFEST_DIR").unwrap()).join("src");
    let out_path = out_dir.join("bindings.rs");
    bindings
        .write_to_file(&out_path)
        .expect("Couldn't write bindings!");

    // it's not a good idea to use `cargo:warning` to notify the user, but is handy
    println!("cargo:warning=bindings.rs generated: {out_path:?}",);
}
