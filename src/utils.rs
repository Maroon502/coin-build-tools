use std::process::Command;
use std::path::Path;

pub fn update_submodules<P: AsRef<Path>>(work_dir: P) {
    let program = "git";
    let args = ["submodule", "update", "--init"];
    println!(
        "Running command: \"{} {}\" in dir: {}",
        program,
        args.join(" "),
        work_dir.as_ref().display()
    );
    let ret = Command::new(program).current_dir(work_dir.as_ref()).args(args).status();

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
        s.push_str(&str);
        s.push_str(";");
    }
    s.pop();
    s
}