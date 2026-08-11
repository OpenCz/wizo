use std::{fs, io, path::{Path, PathBuf}};
use std::error::Error;
use crate::{modules, status};
use crate::modules::parser::GitHubWorkflow;

fn get_github_workflows() -> io::Result<Vec<PathBuf>> {
    let path = Path::new(".github/workflows");

    if !path.exists() {
        return Ok(vec![]);
    }

    let entries = fs::read_dir(path)?;

    let workflows = entries
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "yml" || ext == "yaml" {
                        return Some(path);
                    }
                }
            }
            None
        })
        .collect();

    Ok(workflows)
}

pub fn workflows() -> Option<Vec<GitHubWorkflow>> {
    let mut list_err: Vec<Box<dyn Error>> = Vec::new();
    let mut list_github: Vec<GitHubWorkflow> = Vec::new();

    let pb = status::new_progress("Check workflow files");
    let workflows = match get_github_workflows() {
        Ok(w) => w,
        Err(e) => {
            eprintln!("{e}");
            return None;
        }
    };

    for workflow_path in workflows {
        let path_str = workflow_path.to_string_lossy();

        match modules::parser::workflow_file(&path_str) {
            Ok(workflow_item) => {list_github.push(workflow_item);},
            Err(err) => list_err.push(err),
        }
    }

    pb.finish_and_clear();

    for err in list_err.iter() {
        eprintln!("{err}");
    }
    if list_github.is_empty() || !list_err.is_empty() {
        return None;
    }
    return Some(list_github);
}
