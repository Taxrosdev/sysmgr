use std::{fmt::Debug, fs::read_dir, iter::Skip, path::Path};

pub fn systemd_load_all() {
    // Load all systemd units
    println!("Loading all systemd units...");
    read_dir(Path::new("/usr/lib/systemd/system"))
        .expect("Failed to read systemd units directory")
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry
                .path()
                .extension()
                .map_or(false, |ext| ext == "service")
        })
        .for_each(|entry| {
            if entry
                .file_name()
                .into_string()
                .unwrap()
                .ends_with("@.service")
            {
                // We currently don't support systemd templates
                return;
            };

            let path = entry.path();
            println!("Loading unit: {:?}", path);

            load_unit(&path);
        });
}

struct SystemdUnit {
    Type: String,
    Environment: Option<String>,
    User: Option<String>,
    ExecStart: Option<String>,
    ExecReload: Option<String>,
    SupplementaryGroups: Option<Vec<String>>,
    Description: Option<String>,
    Documentation: Option<String>,
    After: Option<String>,
    WantedBy: Option<String>,
    Wants: Option<String>,
}

fn load_unit(path_to_unit: &Path) -> Result<SystemdUnit, String> {
    let file_name = path_to_unit.file_name().unwrap();

    match std::fs::read_to_string(path_to_unit) {
        Ok(mut content) => {
            println!("Successfully loaded unit: {:?}", file_name);

            content = clean_unit_config(content);

            println!("Unit content: {}", &content);

            Ok(())
        }
        Err(e) => Err(format!("Failed to load unit {:?}: {}", file_name, e)),
    }
}

fn clean_unit_config(content: String) -> String {
    let mut parsed_content : String = "".to_owned();
            
    for mut line in content.lines() {
        // Remove all comments
        let split_line_comment = line.split_once("#");
        
        if split_line_comment.is_some() {
            line = split_line_comment.unwrap().0;
        }

        if line != "" {
            parsed_content = format!("{}\n{}",parsed_content, line);
        }
    };

    parsed_content
}