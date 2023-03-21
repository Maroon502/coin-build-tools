use std::env;

use crate::utils;
use pkg_config;
use vcpkg;

pub fn link_lib_system_if_supported(lib_name: &str) -> bool {
    let host = env::var("HOST").unwrap();
    let target = env::var("TARGET").unwrap();
    let host_and_target_contain = |s| host.contains(s) && target.contains(s);

    if target.contains("msvc") {
        link_windows_msvc_system(lib_name)
    } else if !(host_and_target_contain("apple")
        || host_and_target_contain("freebsd")
        || host_and_target_contain("dragonfly"))
    {
        link_linux_gnu_system(lib_name)
    } else {
        println!(
            "cargo:warning=Can not build {}. Not supported target",
            lib_name
        );
        false
    }
}

fn link_linux_gnu_system(lib_name: &str) -> bool {
    let mut cfg = pkg_config::Config::new();
    cfg.cargo_metadata(true).print_system_libs(false);

    if utils::want_static(lib_name) {
        cfg.statik(true);
    }

    match cfg.probe(lib_name) {
        Ok(lib) => {
            let str = std::env::join_paths(lib.include_paths.iter())
                .unwrap()
                .to_str()
                .unwrap_or_default()
                .to_owned();
            println!("cargo:include={}", str);
            true
        }
        Err(e) => {
            println!("cargo:warining=pkg-config did not find {}: {}", lib_name, e);
            false
        }
    }
}

fn link_windows_msvc_system(lib_name: &str) -> bool {
    if !utils::want_static(lib_name) {
        env::set_var("VCPKGRS_DYNAMIC", "1");
    }

    match vcpkg::Config::new()
        .emit_includes(true)
        .lib_name(lib_name)
        .probe(lib_name)
    {
        Ok(_) => {
            if utils::want_static(lib_name) {
                println!("cargo:rustc-link-lib=static={}", lib_name);
            } else {
                println!("cargo:rustc-link-lib={}", lib_name);
            }
            true
        }
        Err(e) => {
            println!("cargo:warning=vcpkg did not find {}: {}", lib_name, e);
            false
        }
    }
}
