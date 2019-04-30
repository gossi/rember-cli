use walkdir::WalkDir;
use fs_extra::dir;
use fs_extra::copy_items;
pub use super::utils::{create_progress_bar};

pub fn generate_new_application(name: &str) -> Result<(), fs_extra::error::Error> {
    let mut options = dir::CopyOptions::new();
    let mut from_paths = Vec::new();
    let project_name = format!("{}-{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    // Need to figure out how to access these resources in a better way.
    let root_path = format!("{}/registry/src/github.com-1ecc6299db9ec823/{}/fixtures/", env!("CARGO_HOME"), project_name);

    options.copy_inside = true;
    let progress_bar = create_progress_bar(false, "🗄  Generating files...", Some(23));

    for entry in WalkDir::new(root_path) {
        let scaffold_progress = from_paths.len() as u64;
        progress_bar.inc(scaffold_progress);
        let entry = entry.unwrap();
        let path = entry.path().display().to_string();
        from_paths.push(path);
    }

    progress_bar.finish();

    copy_items(&from_paths, format!("{}/", name), &options);

    Ok(())
}
