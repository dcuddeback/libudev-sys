extern crate pkg_config;

use std::env;
use std::fs::File;
use std::path::Path;
use std::process::Command;

use std::io::prelude::*;

fn check_func(function_name: &str, lib: pkg_config::Library) -> bool {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let test_file_name = Path::new(&out_dir).join(format!("check_{}.rs", function_name));

    {
        let mut test_file = File::create(&test_file_name).unwrap();

        writeln!(&mut test_file, "extern \"C\" {{").unwrap();
        writeln!(&mut test_file, "    fn {}();", function_name).unwrap();
        writeln!(&mut test_file, "}}").unwrap();
        writeln!(&mut test_file, "").unwrap();
        writeln!(&mut test_file, "fn main() {{").unwrap();
        writeln!(&mut test_file, "    unsafe {{").unwrap();
        writeln!(&mut test_file, "        {}();", function_name).unwrap();
        writeln!(&mut test_file, "    }}").unwrap();
        writeln!(&mut test_file, "}}").unwrap();
    }
    let rustc = env::var("RUSTC").unwrap();
    let mut cmd = Command::new(rustc);

    cmd.arg(&test_file_name).arg("--out-dir").arg(&out_dir);

    for path in lib.link_paths {
        cmd.arg("-L").arg(path);
    }

    for lib in lib.libs {
        cmd.arg("-l").arg(lib);
    }

    cmd.args(["--target", &std::env::var("TARGET").unwrap()]);
    if let Ok(linker) = std::env::var("RUSTC_LINKER") {
        cmd.args(["-C", &format!("linker={linker}")]);
    }

    let output = cmd.output().unwrap();
    if !output.status.success() {
        println!(
            "cargo:warning=Failed to compile test program for udev function `{}`",
            function_name
        );
        println!("cargo:warning=Using command`{:?}`", cmd);
        println!(
            "cargo:warning=stdout={}",
            String::from_utf8_lossy(&output.stdout)
                .trim()
                .replace('\n', "\ncargo:warning=")
        );
        println!(
            "cargo:warning=stderr={}",
            String::from_utf8_lossy(&output.stderr)
                .trim()
                .replace('\n', "\ncargo:warning=")
        );
        false
    } else {
        true
    }
}

fn main() {
    let lib = pkg_config::probe_library("libudev").unwrap();

    if check_func("udev_hwdb_new", lib) {
        println!("cargo:rustc-cfg=hwdb");
        println!("cargo:hwdb=true");
    } else {
        println!("cargo:hwdb=false");
    }
}
