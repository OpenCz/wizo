use crate::modules::parser::GitHubWorkflow;
use crate::{modules, status};
use std::error::Error;
use std::{
    fs, io,
    path::{Path, PathBuf},
};
use colored::Colorize;

fn get_github_workflows() -> io::Result<Vec<PathBuf>> {
    let path = Path::new(".github/workflows");

    if !path.exists() {
        return Ok(vec![]);
    }

    let entries = fs::read_dir(path)?;

    let workflows = entries
        .filter_map(|entry| {
            let path = entry.ok()?.path();

            if path.is_file()
                && let Some(ext) = path.extension()
                && (ext == "yml" || ext == "yaml")
            {
                return Some(path);
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
    let msg_err: String;

    let workflows = match get_github_workflows() {
        Ok(w) => w,
        Err(e) => {
            eprintln!("{e}");
            
            msg_err = format!("\r{} workflow file structures aren't valid.", "❌".red());
            pb.finish_with_message(msg_err);
            return None;
        }
    };

    for workflow_path in workflows {
        let path_str = workflow_path.to_string_lossy();

        match modules::parser::workflow_file(&path_str) {
            Ok(workflow_item) => {
                list_github.push(workflow_item);
            }
            Err(err) => list_err.push(err),
        }
    }

    if !list_err.is_empty() {
        msg_err = format!("\r{} workflow file structures aren't valid.\n", "❌".red());
        pb.finish_with_message(msg_err);
    }

    for err in list_err.iter() {
        eprintln!("\r{err}");
    }

    if list_github.is_empty() || !list_err.is_empty() {
        return None;
    }
    Some(list_github)
}

pub fn checker() {
    if let Some(_workflow) = workflows() {
        println!("{} Workflow file structures are valid.",
        "✔".green());
    }
}
