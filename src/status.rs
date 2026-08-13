use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::{borrow::Cow, time::Duration};

pub fn new_progress_attach_multi<S>(mp: &MultiProgress, msg: S, color: &str) -> ProgressBar
where
    S: Into<Cow<'static, str>>,
{
    let pb = mp.add(ProgressBar::new_spinner());    
    let template = format!("{{spinner:.{}}} {{wide_msg}}", color);

    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
            .template(&template)
            .unwrap(),
    );

    pb.set_message(msg.into());
    pb.enable_steady_tick(Duration::from_millis(80));
    pb
}

pub fn new_progress<S>(msg: S, color: &str) -> ProgressBar
where
    S: Into<Cow<'static, str>>,
{
    let pb = ProgressBar::new_spinner();
    let template = format!("{{spinner:.{}}} {{wide_msg}}", color);

    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
            .template(&template)
            .unwrap(),
    );

    pb.set_message(msg.into());
    pb.enable_steady_tick(Duration::from_millis(80));
    pb
}
