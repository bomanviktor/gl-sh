use crate::commands::{get_absolute_path, traverse_back, traverse_home};
use crate::consts::{DIR_COLOR, EXECUTABLE_COLOR, SYMLINK_COLOR};
use crate::helpers::command_error;
use crate::helpers::execute::ExecuteOption;
use crate::helpers::execute::ExecuteOption::Out;
use chrono::{DateTime, Local};
use std::fs;
use std::fs::DirEntry;
use std::os::unix::fs::MetadataExt;
use std::os::unix::prelude::PermissionsExt;
use termion::{color, style};
use users::{get_group_by_gid, get_user_by_uid};
use xattr::list;

fn verify_flags(flags: Vec<&str>, flag_a: &mut bool, flag_f: &mut bool, flag_l: &mut bool) {
    let mut flag_buffer = String::new();
    for flag in flags {
        for ch in flag.chars() {
            flag_buffer.push(ch);
        }
    }
    process_flag(&mut flag_buffer, flag_a, flag_f, flag_l);
}

fn process_flag(flag_buffer: &mut str, flag_a: &mut bool, flag_f: &mut bool, flag_l: &mut bool) {
    for ch in flag_buffer.chars() {
        match ch {
            'a' => *flag_a = true,
            'F' => *flag_f = true,
            'l' => *flag_l = true,
            _ => {}
        }
    }
}

fn get_username(uid: u32) -> String {
    if let Some(user) = get_user_by_uid(uid) {
        user.name().to_string_lossy().to_string()
    } else {
        uid.to_string()
    }
}

fn get_groupname(gid: u32) -> String {
    if let Some(group) = get_group_by_gid(gid) {
        group.name().to_string_lossy().to_string()
    } else {
        gid.to_string()
    }
}

fn list_files_l(entry: &DirEntry, path: &str) -> String {
    let metadata = entry.metadata().unwrap();
    let format_path = &format!("{}/{path}", get_absolute_path());
    let xattr = match list(format_path) {
        Ok(attr) => attr.count() != 0,
        _ => false,
    };
    let file_type = entry.file_type().unwrap();

    let permissions = get_permissions_string(metadata.permissions().mode());
    let owner_uid = metadata.uid();
    let owner_name = get_username(owner_uid);
    let group_uid = metadata.gid();
    let group_name = get_groupname(group_uid);
    let size = metadata.len();
    let modification_time = DateTime::<Local>::from(metadata.modified().unwrap());
    let is_symlink = metadata.is_symlink();

    let link_count = metadata.nlink();
    let mut output = String::new();
    output.push_str(&format!(
        "{:<10}{:<1} {:<4} {:>10} {:>10} {:>8} {:>12} ",
        permissions,
        if xattr { "@" } else { "" },
        link_count,
        owner_name,
        group_name,
        size,
        modification_time.format("%b %e %H:%M"),
    ));

    if file_type.is_dir() {
        output.push_str(&format!("{}{}", style::Bold, DIR_COLOR));
    }
    let file_name = entry.file_name().to_string_lossy().to_string();
    if !is_symlink {
        let executable = file_name.ends_with(".sh") || file_name.ends_with(".gsh");
        output.push_str(&format!(
            "{}{}{}{}",
            if executable {
                format!("{}", EXECUTABLE_COLOR)
            } else {
                "".to_string()
            },
            file_name,
            color::Fg(color::Reset),
            style::Reset
        ));
        output
    } else {
        if let Ok(target_path) = fs::read_link(format_path) {
            let target_name = target_path.to_string_lossy().to_string();
            output.push_str(&format!(
                "{}{}{}{}@ -> {target_name}",
                SYMLINK_COLOR,
                file_name,
                color::Fg(color::Reset),
                style::Reset
            ));
        } else {
            eprintln!("Error reading symlink target.");
        }
        output
    }
}

fn get_permissions_string(mode: u32) -> String {
    let mut permissions = String::new();

    // File type
    if mode & 0o170000 == 0o100000 {
        permissions.push('-');
    } else if mode & 0o170000 == 0o040000 {
        permissions.push('d');
    } else {
        permissions.push('l'); // Unknown type
    }

    // Owner permissions
    permissions.push_str(if mode & 0o400 == 0o400 { "r" } else { "-" });
    permissions.push_str(if mode & 0o200 == 0o200 { "w" } else { "-" });
    permissions.push_str(if mode & 0o100 == 0o100 { "x" } else { "-" });

    // Group permissions
    permissions.push_str(if mode & 0o40 == 0o40 { "r" } else { "-" });
    permissions.push_str(if mode & 0o20 == 0o20 { "w" } else { "-" });
    permissions.push_str(if mode & 0o10 == 0o10 { "x" } else { "-" });

    // Other permissions
    permissions.push_str(if mode & 0o4 == 0o4 { "r" } else { "-" });
    permissions.push_str(if mode & 0o2 == 0o2 { "w" } else { "-" });
    permissions.push_str(if mode & 0o1 == 0o1 { "x" } else { "-" });

    permissions
}
pub fn ls(flags: Vec<&str>, args: Vec<&str>) -> ExecuteOption {
    let mut flag_a = false;
    let mut flag_f = false;
    let mut flag_l = false;

    verify_flags(flags, &mut flag_a, &mut flag_f, &mut flag_l);
    let mut output = String::new();
    for arg in &args {
        if args.len() > 1 {
            output.push_str(&format!("{arg}\n:"));
        }
        let mut path = format!("{}/{}", get_absolute_path(), arg);

        if arg.starts_with('~') {
            path = traverse_home(arg);
        }

        let entries = match fs::read_dir(path) {
            Ok(entries) => entries,
            Err(e) => {
                command_error("ls", e, arg);
                continue;
            }
        };

        let mut total = 0;
        let mut sorted_entries = Vec::new();

        for entry in entries.flatten() {
            let file_name = entry.file_name();
            if file_name.to_string_lossy().starts_with('.') && !flag_a {
                continue;
            }
            total += entry.metadata().unwrap().blocks();
            sorted_entries.push(entry);
        }
        // Sort case-insensitively by filename
        sorted_entries.sort_by(|a, b| {
            a.file_name()
                .to_string_lossy()
                .to_lowercase()
                .cmp(&b.file_name().to_string_lossy().to_lowercase())
        });

        if flag_l {
            output.push_str(&format!("total {total}"));
            if flag_a {
                // Get the current directory as a DirEntry
                if let Ok(parent_dir_entries) = fs::read_dir(traverse_back("..")) {
                    let name = get_absolute_path().rsplit_once('/').unwrap().1.to_string();
                    if let Some(entry) = parent_dir_entries
                        .flatten()
                        .find(|e| e.file_name().to_string_lossy() == name)
                    {
                        output.push('\n');
                        let name = format!("{}{name}", color::Fg(color::Cyan));
                        let entry = list_files_l(&entry, ".")
                            .replace(&name, &format!("{}.", color::Fg(color::Cyan)));
                        output.push_str(&entry);
                        if flag_f {
                            output.push('/')
                        }
                    }
                }

                // Get the parent directory as a DirEntry
                if let Ok(parent_dir_entries) = fs::read_dir(traverse_back(".. ..")) {
                    let name = traverse_back("..").rsplit_once('/').unwrap().1.to_string();
                    if let Some(entry) = parent_dir_entries
                        .flatten()
                        .find(|e| e.file_name().to_string_lossy() == name)
                    {
                        output.push('\n');
                        let entry = &list_files_l(&entry, "..");
                        output.push_str(&entry.replace(&name, ".."));
                        if flag_f {
                            output.push('/')
                        }
                    }
                }
            }

            for entry in &sorted_entries {
                output.push('\n');
                let file_name = entry.file_name().to_string_lossy().to_string();
                output.push_str(&list_files_l(entry, &file_name));
                let file_type = entry.file_type().unwrap();
                if file_type.is_dir() && flag_f {
                    output.push('/');
                } else if file_name.ends_with(".sh") || file_name.ends_with(".gsh") {
                    output.push('*');
                }
            }
            continue;
        }
        let reset_color = color::Fg(color::Reset);
        let dir_color = DIR_COLOR;
        let script_color = EXECUTABLE_COLOR;

        if flag_a {
            output.push_str(&format!(
                "{dir_color}.{reset_color}{}\t",
                if flag_f { "/" } else { "" }
            ));
            output.push_str(&format!(
                "{dir_color}..{reset_color}{}\t",
                if flag_f { "/" } else { "" }
            ));
        }

        for entry in sorted_entries {
            let file_name = entry.file_name().to_string_lossy().to_string();
            let file_type = entry.file_type().unwrap();

            if file_type.is_file() {
                if file_name.ends_with(".gsh") || file_name.ends_with(".sh") {
                    output.push_str(&format!("{script_color}{file_name}{reset_color}{} ",
                    if flag_a {"*"} else {""}));
                } else {
                    output.push_str(&format!("{file_name} ",));
                }
            } else {
                let formatted_string = format!(
                    "{}{}{}{}{}{}",
                    style::Bold,
                    color::Fg(color::Cyan),
                    file_name,
                    color::Fg(color::Reset),
                    style::Reset,
                    if flag_f { "/" } else { "" }
                );

                output.push_str(&formatted_string);
                output.push('\t');
            }
        }
    }
    Out(output)
}
