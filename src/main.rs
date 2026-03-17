use termion::terminal_size_pixels;
use include_dir::{include_dir, Dir};
use fastrand;
use std::{process::Command, env, path::PathBuf};

static SOYJAKS_DIR: Dir = include_dir!("soyjaks");

fn main() {
    let cwd = env::current_dir().unwrap();

    let files: Vec<String> = SOYJAKS_DIR
        .files()
        .map(|f| {
            let p = f.path();
            p.strip_prefix("soyjaks").unwrap_or(p).to_string_lossy().into_owned()
        })
        .collect();

    let file_relative = &files[fastrand::usize(0..files.len())];
    let absolute_path: PathBuf = cwd.join("soyjaks").join(file_relative);

    let (_x, y) = terminal_size_pixels().unwrap();
    
    let original = imagesize::size(&absolute_path).unwrap();
    let max_height = y / 2;
    let (mut width, mut height) = (original.width, original.height);
    if height > max_height.into() {
        let scale = max_height as f64 / height as f64;
        height = max_height as usize;
        width = (width as f64 * scale).round() as usize;
    }
    Command::new("img2sixel")
        .arg("-w")
        .arg(width.to_string())
        .arg("-h")
        .arg(height.to_string())
        .arg(absolute_path)
        .status()
        .expect("failed to run img2sixel, do you have libsixel installed?");
}

