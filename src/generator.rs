pub mod generator {
    use serde_json::Result;
    use std::{
        fs::{self},
        path::Path,
        process::Command,
    };

    pub fn generate(mut path: String) -> std::io::Result<()> {
        create_new_cargo_project(&path);
        fs::create_dir_all(&path)?;
        if !path.ends_with("/") {
            path = path + "/";
        }
        generate_dirs(&path).unwrap();
        // generate_structure();

        fs::File::create(String::from(&path) + "src/controllers/home.controller.rs")?;

        fs::File::create(String::from(&path) + "src/views/home.view.rs")?;
        Ok(())
    }

    fn create_new_cargo_project(path: &String) {
        let splitted_path = path.rsplit_once("/").unwrap();
        let dir_path = splitted_path.0.to_owned() + "/";
        let dir_name = splitted_path.1;

        Command::new("cargo")
            .current_dir(dir_path)
            .args(["new", dir_name])
            .output()
            .expect("Can't create the folder");
    }

    fn generate_structure(path: &str) -> Result<bool> {
        generate_dirs(path).unwrap();
        
        Ok(true)
    }

    fn generate_dirs(path: &str) -> Result<bool> {
        let ref_path = Path::new("./src/references/dir_ref.json");
        let dir_refs = fs::read_to_string(ref_path).expect("Unable to read dir_ref file");

        let dir_refs_json: Vec<&str> = serde_json::from_str(&dir_refs)?;

        for reference in dir_refs_json {
            fs::create_dir_all(String::from(path) + "src/" + reference).unwrap();
        }
        Ok(true)
    }
}
