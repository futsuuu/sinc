#[cfg(not(target_os = "windows"))]
use std::os::unix;
#[cfg(target_os = "windows")]
use std::os::windows;
use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
    process::{Command, Output, Stdio},
};

use fs_extra::{self, dir::CopyOptions};
use thiserror::Error;
#[cfg(target_os = "windows")]
use windows_sys::Win32::UI::Shell::IsUserAnAdmin;

use crate::ui;

type Result<T> = std::result::Result<T, SyncError>;

#[derive(Error, Debug)]
enum SyncError {
    #[error("File system processing failed")]
    FsProcessingError,
    #[error("Command execution failed")]
    CommandError,
    #[error("{} is not supported in your environment", ui::item_type(error_type))]
    Unsupported { error_type: String },
    #[error("You should run with administrator rights")]
    NoAdministrator,
    #[error("{} and {} not found", ui::path(path0.as_path()), ui::path(path1.as_path()))]
    NotFound { path0: PathBuf, path1: PathBuf },
}

#[derive(Debug)]
pub struct Dotfile {
    name: String,
    path: PathBuf,
    target: PathBuf,
    sync_type: String,
    enable: bool,
    hook_add: String,
}

impl Dotfile {
    pub fn new(
        name: String,
        path: String,
        target: String,
        sync_type: String,
        enable: bool,
        hook_add: String,
    ) -> Self {
        let path = PathBuf::from(path);
        let target = PathBuf::from(target);
        Self {
            name,
            path,
            target,
            sync_type,
            enable,
            hook_add,
        }
    }

    pub fn sync(&self) {
        print!("{}", ui::title(self.name.clone()));
        if self.enable {
            if let Err(e) = self._sync() {
                println!("{}", e)
            }
        } else {
            self.print_message()
        }
    }

    fn _sync(&self) -> Result<()> {
        if self.target.exists() {
            if self.path.exists() {
                // o -> o
                let target = &self.target;
                let new_target = PathBuf::from(self.target.display().to_string() + ".old_version");

                fs::rename(&self.target, &new_target).or(Err(SyncError::FsProcessingError))?;
                match self.new_item() {
                    Ok(_) => {
                        fs_extra::remove_items(&[&new_target]).or(Err(SyncError::FsProcessingError))
                    }
                    Err(e) => {
                        fs::rename(new_target, target).or(Err(SyncError::FsProcessingError))?;
                        Err(e)
                    }
                }?;
            } else {
                // x -> o
                let parent_dir = self.path.parent().unwrap();
                if !parent_dir.exists() {
                    fs::create_dir_all(parent_dir).or(Err(SyncError::FsProcessingError))?;
                }
                fs::rename(&self.target, &self.path).or(Err(SyncError::FsProcessingError))?;
                self.new_item()?;
            };
        } else if self.path.exists() {
            // o -> x
            self.new_item()?;
        } else {
            return Err(SyncError::NotFound {
                path0: self.path.clone(),
                path1: self.target.clone(),
            });
        };
        if !&self.hook_add.is_empty() {
            let output = run_command(&self.hook_add)?;
            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
            println!("{}", &output.status);
        }

        Ok(())
    }

    pub fn print_message(&self) {
        if self.enable {
            println!(
                "{}{}{}{}{}",
                ui::path(&self.path),
                ui::symbol(" ====( "),
                ui::item_type(self.sync_type.clone()),
                ui::symbol(" )===> "),
                ui::path(&self.target),
            )
        } else {
            println!("disable: {}", ui::path(&self.path));
        }
    }

    fn new_item(&self) -> Result<()> {
        match self.sync_type.as_str() {
            "symlink" => {
                self.create_symlink()?;
            }
            "hardlink" => {
                fs::hard_link(&self.path, &self.target).or(Err(SyncError::FsProcessingError))?;
            }
            "junction" => {
                if cfg!(target_os = "windows") {
                    Command::new("cmd")
                        .arg("/C")
                        .args([
                            "mklink",
                            "/J",
                            self.target.to_str().unwrap(),
                            self.path.to_str().unwrap(),
                        ])
                        .stdout(Stdio::null())
                        .spawn()
                        .or(Err(SyncError::CommandError))?;
                } else {
                    return Err(SyncError::Unsupported {
                        error_type: self.sync_type.clone(),
                    });
                }
            }
            "copy" => {
                fs_extra::copy_items(
                    &[&self.path],
                    &self.target,
                    &CopyOptions::new().copy_inside(true),
                )
                .or(Err(SyncError::FsProcessingError))?;
            }
            _ => (),
        }
        self.print_message();
        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    fn create_symlink(&self) -> Result<()> {
        unix::fs::symlink(&self.path, &self.target).or(Err(SyncError::FsProcessingError))?;
        Ok(())
    }

    #[cfg(target_os = "windows")]
    fn create_symlink(&self) -> Result<()> {
        unsafe {
            if IsUserAnAdmin() == 0 {
                return Err(SyncError::NoAdministrator);
            }
        }
        if self.target.is_dir() {
            windows::fs::symlink_dir(&self.path, &self.target)
        } else {
            windows::fs::symlink_file(&self.path, &self.target)
        }
        .or(Err(SyncError::FsProcessingError))?;
        Ok(())
    }
}

fn run_command(command: &str) -> Result<Output> {
    let shell = if cfg!(target_os = "windows") {
        ("cmd", "/C")
    } else {
        ("sh", "-c")
    };
    let output = Command::new(shell.0)
        .arg(shell.1)
        .arg(command)
        .output()
        .or(Err(SyncError::CommandError))?;
    Ok(output)
}
