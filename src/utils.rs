use std::env;
use std::path::Path;
use std::process::Command;

pub fn update_submodules<P: AsRef<Path>>(work_dir: P) {
    let program = "git";
    let args = ["submodule", "update", "--init"];
    println!(
        "Running command: \"{} {}\" in dir: {}",
        program,
        args.join(" "),
        work_dir.as_ref().display()
    );
    let ret = Command::new(program)
        .current_dir(work_dir.as_ref())
        .args(args)
        .status();

    match ret.map(|status| (status.success(), status.code())) {
        Ok((true, _)) => (),
        Ok((false, Some(c))) => panic!("Command failed with error code {}", c),
        Ok((false, None)) => panic!("Command got killed"),
        Err(e) => panic!("Command failed with error: {}", e),
    }
}

pub fn cat_strs(strs: &[String]) -> String {
    let mut s = String::new();
    for str in strs {
        s.push_str(str);
        s.push(';');
    }
    s.pop();
    s
}

pub fn want_static(lib_name: &str) -> bool {
    env::var_os(format!("CARGO_{}_STATIC", lib_name.to_ascii_uppercase())).unwrap_or_default()
        == "1"
}

pub fn want_system(lib_name: &str) -> bool {
    env::var_os(format!("CARGO_{}_SYSTEM", lib_name.to_ascii_uppercase())).unwrap_or_default()
        == "1"
}
