use std::{path::PathBuf, process::Command};

pub fn execute(root_dir : PathBuf, exec_path : String) {
    let full_path = format!("{}\\{}",project_root::get_project_root().unwrap().display(),root_dir.display());
    //println!("{}",full_path);
    if exec_path.ends_with(".jar") {

        let output = Command::new("cmd")
            .args(&["/C",format!("cd {} && java -jar {}", full_path, exec_path).as_str()])
            .output()
            .expect("Failed to execute process");

        println!("{:?}",String::from_utf8(output.stderr));
    } else {
        let output = Command::new("cmd")
            .args(&["/C",format!("cd {} && {}", full_path, exec_path).as_str()])
            .output()
            .expect("Failed to execute process");
            
        println!("{:?}",String::from_utf8(output.stderr));
    }
}