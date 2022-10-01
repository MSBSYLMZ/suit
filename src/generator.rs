pub mod generator {
    use std::{
        fs::{self},
        process::{Command, Stdio},
    };

    pub fn generate(mut path: String) -> std::io::Result<()> {
        create_new_cargo_project(&path);
        fs::create_dir_all(&path)?;
        if !path.ends_with("/") {
            path = path + "/";
        }
        fs::create_dir_all(String::from(&path) + "src/controller")?;
        fs::create_dir_all(String::from(&path) + "src/model")?;
        fs::create_dir_all(String::from(&path) + "src/view")?;
        Ok(())
    }

    fn create_new_cargo_project(path: &String) {
        let splitted_path = path.rsplit_once("/").unwrap();
        let dir_path = splitted_path.0.to_owned() + "/";
        let dir_name = splitted_path.1;

        let mut cmd = Command::new("cargo")
            .current_dir(dir_path)
            .args(["new", dir_name])
            .output()
            .expect("Can't create the folder");
        println!("{:?}", cmd);
    }
}
