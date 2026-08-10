use std::{fs, io, path::Path};

use crate::{modules, status};

fn get_github_workflows() -> io::Result<Vec<String>> {
    let path = Path::new(".github/workflows");

    if !path.exists() {
        return Ok(vec![]);
    }

    let entries = fs::read_dir(path)?;

    let workflows = entries
        .filter_map(|entry| {
            let path = entry.ok()?.path();

            if path.is_file() {
                let ext = path.extension()?.to_str()?;
                if ext == "yml" || ext == "yaml" {
                    return Some(path.display().to_string());
                }
            }
            None
        })
        .collect();

    Ok(workflows)
}

pub fn handle(dry_run: bool, _jobs: u32) {
    let workflows = get_github_workflows();
    let pb = status::new_progress("Parse all workflow files");

    for workflow in workflows.unwrap_or_default() {
        let workflow_item = modules::parser::workflow_file(&workflow);

        println!("{:?}",workflow_item.unwrap().on);
    }
    pb.finish_and_clear();
    if dry_run {
        return;
    }
}
