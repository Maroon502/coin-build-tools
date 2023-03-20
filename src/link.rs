use std::env;

use pkg_config;
use vcpkg;

pub fn link_lib_system_if_defined(lib_name: &str) -> bool {
    if want_system(lib_name) {
        link_lib_system_if_surported(lib_name)
    } else {
        false
    }
}


pub fn link_lib_system_if_surported(lib_name: &str) -> bool {
    let host = env::var("HOST").unwrap();
    let target = env::var("TARGET").unwrap();
    let host_and_target_contain = |s| host.contains(s) && target.contains(s);

    if target.contains("msvc") {
        link_windows_msvc_system(lib_name)
    } else if !(host_and_target_contain("apple") ||
        host_and_target_contain("freebsd") ||
        host_and_target_contain("dragonfly"))
    {
        link_linux_gnu_system(lib_name)
    } else {
        false
    }
}

fn link_linux_gnu_system(lib_name: &str) -> bool{
    let mut cfg = pkg_config::Config::new();
    cfg.cargo_metadata(true)
        .print_system_libs(false);

    if want_static(lib_name) {
        cfg.statik(true);
    }

    match cfg.probe(lib_name) {
        Ok(lib) => {
            for include in lib.include_paths {
                println!("cargo:include={}", include.display());
            }
            true
        }
        Err(e) => {
            println!("pkg-config did not find {}: {}", lib_name, e);
            false
        }
    }
}

fn link_windows_msvc_system(lib_name: &str) -> bool{
    if !want_static(lib_name) {
        env::set_var("VCPKGRS_DYNAMIC", "1");
    }

    match vcpkg::Config::new()
        .emit_includes(true)
        .lib_name(lib_name)
        .probe(lib_name)
    {
        Ok(_) => {
            if want_static(lib_name) {
                println!("cargo:rustc-link-lib=static={}", lib_name);
            } else {
                println!("cargo:rustc-link-lib={}", lib_name);
            }
            true
        }
        Err(e) => {
            println!("vcpkg did not find {}: {}", lib_name, e);
            false
        }
    }
}

fn want_static(lib_name: &str) -> bool {
    env::var_os(format!("CARGO_{}_STATIC", lib_name.to_ascii_uppercase())).unwrap_or_default() == "1"
}

fn want_system(lib_name: &str) -> bool {
    env::var_os(format!("CARGO_{}_SYSTEM", lib_name.to_ascii_uppercase())).unwrap_or_default() == "1"
}
