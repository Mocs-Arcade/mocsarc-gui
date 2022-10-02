use std::{path::PathBuf, process::Command};

pub fn execute(root_dir : PathBuf, exec_path : String) {
    let full_path = format!("{}\\{}\\{}",project_root::get_project_root().unwrap().display(),root_dir.display(),exec_path);
    //println!("{}",full_path);
    if exec_path.ends_with(".jar") {
        let java = Command::new("cmd")
            .args(&["/C","java -version"])
            .output()
            .expect("Failed to execute process");

            println!("{:?}",String::from_utf8(java.stderr));

        let _output = Command::new("cmd")
            .args(&["/C",format!("java -jar {}", full_path).as_str()])
            .output()
            .expect("Failed to execute process");
    } else {
        let _output = Command::new("cmd")
            .args(&["/C",format!("{}", full_path).as_str()])
            .output()
            .expect("Failed to execute process");
    }
}