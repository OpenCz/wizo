use indicatif::{MultiProgress, ProgressBar};

use crate::{commands::check::workflows, modules::parser::GitHubWorkflow, status};
use core::time;
use std::{
    sync::{self, Arc},
    thread,
};

// fn set_time_duration(msg: String) {}

fn launch_workflow(workflow: &Arc<GitHubWorkflow>, pb: &ProgressBar) {
    let data = sync::Mutex::from(workflow);
    let _res = data.try_lock();

    thread::sleep(time::Duration::from_secs(rand::random_range(0..13)));
    let msg = format!(
        "Job `{}` done...",
        workflow.name.as_deref().unwrap_or_default()
    );
    pb.finish_with_message(msg);
}

pub fn handle(dry_run: bool, _jobs: u32) {
    if let Some(workflows) = workflows() {
        if dry_run {
            return;
        }

        let mb = MultiProgress::new();
        let mut handles = vec![];

        for workflow in workflows {
            let workflow = Arc::new(workflow);
            let name = format!(
                "Waiting `{}` ...",
                workflow.name.as_deref().unwrap_or_default()
            );
            let pb = status::new_progress_attach_multi(&mb, name);
            let subthread = thread::spawn(move || {
                launch_workflow(&workflow, &pb);
            });

            handles.push(subthread);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}
