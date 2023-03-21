use std::env;
use cc::Build;

use crate::utils;

pub fn init_builder() -> Build{
    let target = env::var("TARGET").unwrap();

    let mut builder = cc::Build::new()
            .cpp(true)
            .warnings(false)
            .extra_warnings(false)
            .define("NDEBUG", None)
            .define("HAVE_STDIO_H", None)
            .define("HAVE_STDLIB_H", None)
            .define("HAVE_STRING_H", None)
            .define("HAVE_INTTYPES_H", None)
            .define("HAVE_STDINT_H", None)
            .define("HAVE_STRINGS_H", None)
            .define("HAVE_SYS_TYPES_H", None)
            .define("HAVE_SYS_STAT_H", None)
            .define("HAVE_UNISTD_H", None)
            .define("HAVE_CMATH", None)
            .define("HAVE_CFLOAT", None)
            .define("HAVE_DLFCN_H", None)
            .define("HAVE_MEMORY_H", None)
            .to_owned();

    if target.contains("msvc") {
        builder.flag("-EHsc")
            .flag_if_supported("-std:c++11");
    } else {
        builder.flag("-std=c++11")
            .flag("-w");
    }
    builder
}


pub fn get_metedata_from(lib_name: &str) -> (Vec<String>, Vec<String>) {
    let includes_dir = if let Some(paths) = env::var_os(&format!("DEP_{}_INCLUDE", lib_name.to_ascii_uppercase())) {
        env::split_paths(&paths).map(|p| format!("{}", p.display())).collect()
    } else {
        Vec::new()
    };

    let coinflags = if let Ok(flags) = env::var(format!("DEP_{}_COINFLAGS", lib_name.to_ascii_uppercase())) {
        flags.split(';').map(|f| f.to_string()).collect()
    } else {
        Vec::new()
    };

    (includes_dir, coinflags)
}

pub fn print_metedata(includes: Vec<String>, coinflags: Vec<String>) {
    if !includes.is_empty() {
        let include_str = env::join_paths(includes.iter()).unwrap();
        println!("cargo:include={}", include_str.to_str().unwrap());
    }
    if !coinflags.is_empty() {
        let coinflags_str = utils::cat_strs(coinflags.as_slice());
        println!("cargo:coinflags={}", coinflags_str);
    }
}

