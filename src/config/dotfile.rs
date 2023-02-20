#[cfg(target_os = "unix")]
use std::os::unix;
#[cfg(target_os = "windows")]
use std::os::windows;
use std::{
    fs,
    io::Error,
    path::PathBuf,
    process::{Command, Stdio},
};

use fs_extra::{self, dir::CopyOptions};

#[derive(Debug)]
pub struct Dotfile {
    path: PathBuf,
    target: PathBuf,
    sync_type: SyncType,
}

#[derive(Debug)]
pub enum SyncType {
    SymLink,
    HardLink,
    Junction,
    Copy,
}

impl Dotfile {
    pub fn new(path: String, target: String, sync_type: Option<&str>) -> Self {
        Self {
            path: PathBuf::from(path),
            target: PathBuf::from(target),
            sync_type: match sync_type {
                Some("symlink") => SyncType::SymLink,
                Some("hardlink") => SyncType::HardLink,
                Some("junction") => SyncType::Junction,
                Some("copy") => SyncType::Copy,
                _ => SyncType::Copy,
            },
        }
    }

    pub fn sync(&self) -> Result<(), Error> {
        if self.target.exists() {
            if self.path.exists() {
                fs_extra::remove_items(&[&self.target]).unwrap();
            } else {
                let parent_dir = self.path.parent().unwrap();
                if !parent_dir.exists() {
                    fs::create_dir_all(parent_dir)?;
                }
                fs::rename(&self.target, &self.path)?;
            }
        }

        if self.path.exists() {
            match self.sync_type {
                SyncType::SymLink => {
                    self.create_symlink()?;
                }
                SyncType::HardLink => {
                    fs::hard_link(&self.path, &self.target)?;
                }
                SyncType::Junction => {
                    if cfg!(target_os = "windows") {
                        let _ = Command::new("cmd")
                            .arg("/C")
                            .args([
                                "mklink",
                                "/J",
                                self.target.to_str().unwrap(),
                                self.path.to_str().unwrap(),
                            ])
                            .stdout(Stdio::null())
                            .spawn();
                    }
                }
                SyncType::Copy => {
                    fs_extra::copy_items(
                        &[&self.path],
                        &self.target,
                        &CopyOptions::new().copy_inside(true),
                    )
                    .unwrap();
                }
            }
        }

        Ok(())
    }

    #[cfg(target_os = "unix")]
    fn create_symlink(&self) -> Result<(), Error> {
        unix::fs::symlink(&self.path, &self.target)
    }

    #[cfg(target_os = "windows")]
    fn create_symlink(&self) -> Result<(), Error> {
        if self.target.is_dir() {
            windows::fs::symlink_dir(&self.path, &self.target)
        } else {
            windows::fs::symlink_file(&self.path, &self.target)
        }
    }
}
