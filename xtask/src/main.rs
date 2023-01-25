use std::{
    path::{Path, PathBuf},
    process::Command,
};

use once_cell::sync::OnceCell;

fn main() {}

#[cfg(test)]
fn cargo<I, S>(args: I) -> Command
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let mut cmd = Command::new("cargo");
    cmd.args(args);
    cmd.current_dir(get_cargo_workspace());
    cmd
}

pub fn walk_dir<'a>(
    root: &'_ Path,
    skip: &'static [impl AsRef<std::ffi::OsStr> + Send + Sync + 'a],
    ext: impl for<'s> Fn(Option<&'s std::ffi::OsStr>) -> bool + Sync + Send + 'static,
) -> impl Iterator<Item = Result<ignore::DirEntry, ignore::Error>> {
    ignore::WalkBuilder::new(root)
        .hidden(false)
        .filter_entry(move |e| {
            if skip
                .iter()
                .map(|s| -> &std::ffi::OsStr { s.as_ref() })
                .any(|dir| e.file_name() == dir)
            {
                return false;
            } else if e.file_type().map_or(false, |f| f.is_dir()) {
                return true;
            }
            ext(e.path().extension())
        })
        .build()
}

/// Returns the cargo workspace for the manifest
pub fn get_cargo_workspace() -> &'static Path {
    static WORKSPACE: OnceCell<PathBuf> = OnceCell::new();
    #[derive(Debug, serde::Deserialize)]
    pub struct CargoMetadata {
        pub workspace_root: PathBuf,
    }
    WORKSPACE
        .get_or_try_init(|| {
            let mut cmd = Command::new("cargo");
            cmd.args(["metadata", "--format-version", "1", "--no-deps"]);
            cmd.current_dir(env!("CARGO_MANIFEST_DIR"));
            let out = cmd.output()?;
            if out.status.success() {
                Ok(String::from_utf8(out.stdout)
                    .map_err(eyre::Report::from)
                    .and_then(|s| serde_json::from_str::<CargoMetadata>(&s).map_err(Into::into))?
                    .workspace_root)
            } else {
                eyre::bail!("metadata failed: {}", String::from_utf8_lossy(&out.stderr))
            }
        })
        .unwrap()
}

#[test]
pub fn formatted() {
    assert!(cargo(["fmt", "--", "--check"]).status().unwrap().success());
}

#[test]
pub fn deny() {
    if cargo(["deny", "-V"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
    {
        assert!(cargo(["deny", "check"]).status().unwrap().success());
    }
}

#[test]
pub fn ends_with_newline() -> Result<(), eyre::Report> {
    for file in walk_dir(get_cargo_workspace(), &[".git", "target"], |_| true) {
        let file = file?;
        if !file.file_type().map_or(true, |f| f.is_file()) {
            continue;
        }
        eprintln!("File: {:?}", file.path());
        assert!(
            std::fs::read_to_string(file.path())
                .unwrap_or_else(|_| String::from("\n"))
                .ends_with('\n'),
            "file {:?} does not end with a newline",
            file.path().display()
        );
    }
    Ok(())
}
