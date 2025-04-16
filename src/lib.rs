pub mod actions;
pub mod config;
pub mod filters;
pub mod utils;

use walkdir::WalkDir;

pub fn run(config: &config::Config) {
    for rule in config.rules.iter() {
        process_rule(&rule);
    }
}

fn process_rule(rule: &config::Rule) {
    let filters = filters::parse(&rule.filters);
    let action = actions::parse(&rule.action);
    let mut matches = vec![];

    for location in &rule.locations {
        let walker = if rule.recursive {
            WalkDir::new(location)
        } else {
            WalkDir::new(location).max_depth(rule.max_depth.unwrap_or(1))
        };

        for entry in walker.into_iter().filter_map(Result::ok) {
            let file_meta_data = utils::FileMetaData::from_path(entry.path()).unwrap();

            if file_meta_data.is_dir {
                continue;
            }

            let mut filters_result = false;
            for f in &filters {
                filters_result = f.matches(&file_meta_data);
                if !filters_result {
                    break;
                }
            }
            if !filters_result {
                continue;
            }

            matches.push(file_meta_data);
        }
    }

    println!("{0} : ", rule.name);
    if matches.is_empty() {
        println!("No matching files found.");
        return;
    }

    if let Err(errors) = action.execute(&matches) {
        println!("Action completed with errors : ");
        for error in errors {
            eprintln!("Error: {}", error);
        }
        return;
    }
    println!("Action completed successfully.")
}
