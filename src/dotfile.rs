#[cfg(not(target_os = "windows"))]
use std::os::unix;
#[cfg(target_os = "windows")]
use std::os::windows;
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, ExitStatus, Stdio},
};

use fs_extra::{self, dir::CopyOptions};
use thiserror::Error;
#[cfg(target_os = "windows")]
use windows_sys::Win32::UI::Shell::IsUserAnAdmin;

use crate::{path::cache_file, ui};

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
            if let Err(e) = sync(&self.path, &self.target, &self.sync_type) {
                println!("{}", e)
            } else {
                print_message(&self.path, &self.target, &self.sync_type);

                if !&self.hook_add.is_empty() {
                    match run_command(&self.hook_add) {
                        Ok(exit_status) => {
                            if let Some(code) = exit_status.code() {
                                println!("Exit with status code {}.", code);
                            } else {
                                println!("Process terminated by signal.");
                            }
                        }
                        Err(e) => println!("{}", e),
                    }
                }
            }
        } else {
            println!("disable: {}", ui::path(&self.path));
        }
    }
}

fn print_message(path: &Path, target: &Path, sync_type: &str) {
    println!(
        "{}{}{}{}{}",
        ui::path(path),
        ui::symbol(" ====( "),
        ui::item_type(sync_type),
        ui::symbol(" )===> "),
        ui::path(target),
    )
}

fn sync(path: &PathBuf, target: &PathBuf, sync_type: &str) -> Result<()> {
    if target.exists() {
        if path.exists() {
            // o -> o
            let target = &target;
            let new_target = PathBuf::from(target.display().to_string() + ".old_version");

            fs::rename(target, &new_target).or(Err(SyncError::FsProcessingError))?;
            match new_item(path, target, sync_type) {
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
            let parent_dir = path.parent().unwrap();
            if !parent_dir.exists() {
                fs::create_dir_all(parent_dir).or(Err(SyncError::FsProcessingError))?;
            }
            fs::rename(target, path).or(Err(SyncError::FsProcessingError))?;
            new_item(path, target, sync_type)?;
        };
    } else if path.exists() {
        // o -> x
        new_item(path, target, sync_type)?;
    } else {
        return Err(SyncError::NotFound {
            path0: path.clone(),
            path1: target.clone(),
        });
    };

    Ok(())
}

fn new_item(path: &PathBuf, target: &PathBuf, sync_type: &str) -> Result<()> {
    match sync_type {
        "symlink" => {
            create_symlink(path, target)?;
        }
        "hardlink" => {
            fs::hard_link(path, target).or(Err(SyncError::FsProcessingError))?;
        }
        "junction" => {
            if cfg!(target_os = "windows") {
                Command::new("cmd")
                    .arg("/C")
                    .args([
                        "mklink",
                        "/J",
                        target.to_str().unwrap(),
                        path.to_str().unwrap(),
                    ])
                    .stdout(Stdio::null())
                    .spawn()
                    .or(Err(SyncError::CommandError))?;
            } else {
                return Err(SyncError::Unsupported {
                    error_type: sync_type.to_string(),
                });
            }
        }
        "copy" => {
            fs_extra::copy_items(&[path], target, &CopyOptions::new().copy_inside(true))
                .or(Err(SyncError::FsProcessingError))?;
        }
        _ => (),
    }
    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn create_symlink(path: &PathBuf, target: &PathBuf) -> Result<()> {
    unix::fs::symlink(path, target).or(Err(SyncError::FsProcessingError))?;
    Ok(())
}

#[cfg(target_os = "windows")]
fn create_symlink(path: &PathBuf, target: &PathBuf) -> Result<()> {
    unsafe {
        if IsUserAnAdmin() == 0 {
            return Err(SyncError::NoAdministrator);
        }
    }
    if target.is_dir() {
        windows::fs::symlink_dir(path, target)
    } else {
        windows::fs::symlink_file(path, target)
    }
    .or(Err(SyncError::FsProcessingError))?;
    Ok(())
}

fn run_command(command: &str) -> Result<ExitStatus> {
    let (command, script_name) = if cfg!(target_os = "windows") {
        (format!("@echo off\n{}", command), "hook_script.bat")
    } else {
        (command.to_string(), "hook_script")
    };
    let script_path = cache_file(script_name).or(Err(SyncError::FsProcessingError))?;
    let mut file = fs::File::create(&script_path).or(Err(SyncError::FsProcessingError))?;

    file.write_all(command.as_bytes())
        .or(Err(SyncError::FsProcessingError))?;

    let (program, arg) = {
        let script_path_str = script_path.to_str().unwrap();
        if cfg!(target_os = "windows") {
            (script_path_str, "")
        } else {
            ("sh", script_path_str)
        }
    };
    let exit_status = Command::new(program)
        .arg(arg)
        .spawn()
        .or(Err(SyncError::CommandError))?
        .wait()
        .or(Err(SyncError::CommandError))?;
    Ok(exit_status)
}
