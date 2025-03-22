pub mod filters;
pub mod actions;
pub mod config;

use crate::config::Action::{ Move };
use walkdir::{WalkDir};

pub fn run(config: &config::Config) {
    for rule in config.rules.iter() {
        process_rule(&rule);
    }
}

fn process_rule(rule: &config::Rule) {
    let mut matches = vec![];
    let filters = process_filters(&rule.filters);

    for location in &rule.locations {
        let walker = if rule.recursive {
            WalkDir::new(location)
        } else {
            WalkDir::new(location).max_depth(rule.max_depth.unwrap_or(1))
        };

        for entry in walker.into_iter().filter_map(Result::ok) {
            let file_path = entry.path();

            if file_path.is_dir() {
                continue;
            }

            let mut filters_result = false;
            for f in &filters {
                filters_result = f.matches(file_path);
                if !filters_result { break }
            }
            if !filters_result { continue }

            matches.push(file_path.to_path_buf());
        }
    }

    println!("{0} : ",rule.name);

    if matches.is_empty() {
        println!("No matching files found.");
        return
    }

    println!("{matches:#?}");
}

fn process_filters(filters_cfg: &config::Filters) -> Vec<Box<dyn filters::Filter + '_>> {
    let mut filters: Vec<Box<dyn filters::Filter>> = Vec::new();

    if let Some(ref exts) = filters_cfg.extensions {
        filters.push(Box::new(filters::ExtensionFilter::new(exts,false)));
    } else if let Some(ref not_exts) = filters_cfg.not_extensions {
        filters.push(Box::new(filters::ExtensionFilter::new(not_exts,true)));
    }

    if let Some(ref name_filter) = filters_cfg.name {
        filters.push(Box::new(filters::NameFilter::new(name_filter)));
    }

    filters
}
