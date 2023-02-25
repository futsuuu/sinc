#[cfg(not(target_os = "windows"))]
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
    sync_type: String,
    enable: bool,
}

impl Dotfile {
    pub fn new(path: String, target: String, sync_type: String, enable: bool) -> Self {
        let path = PathBuf::from(path);
        let target = PathBuf::from(target);
        let enable = enable & path.exists() & target.exists();
        Self {
            path,
            target,
            sync_type,
            enable,
        }
    }

    pub fn sync(&self) -> Result<(), Error> {
        if self.enable {
            self._sync()?;
        }
        Ok(())
    }

    fn _sync(&self) -> Result<(), Error> {
        if self.target.exists() {
            if self.path.exists() {
                // o -> o
                let target = &self.target;
                let new_target = PathBuf::from(self.target.display().to_string() + ".old_version");

                fs::rename(&self.target, &new_target)?;
                match self.new_item() {
                    Ok(_) => fs_extra::remove_items(&[&new_target]).unwrap(),
                    Err(_) => fs::rename(new_target, target)?,
                }
            } else {
                // x -> o
                let parent_dir = self.path.parent().unwrap();
                if !parent_dir.exists() {
                    fs::create_dir_all(parent_dir)?;
                }
                fs::rename(&self.target, &self.path)?;
                self.new_item()?;
            }
        } else if self.path.exists() {
            // o -> x
            self.new_item()?;
        }

        Ok(())
    }

    pub fn get_message(&self) -> String {
        if self.enable {
            format!(
                "{} ===( {} )==> {}",
                self.path.display(),
                self.sync_type,
                self.target.display()
            )
        } else {
            format!("disable: {}", self.path.display())
        }
    }

    fn new_item(&self) -> Result<(), Error> {
        match self.sync_type.as_str() {
            "symlink" => {
                self.create_symlink()?;
            }
            "hardlink" => {
                fs::hard_link(&self.path, &self.target)?;
            }
            "junction" => {
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
            "copy" => {
                fs_extra::copy_items(
                    &[&self.path],
                    &self.target,
                    &CopyOptions::new().copy_inside(true),
                )
                .unwrap();
            }
            _ => (),
        }
        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
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
