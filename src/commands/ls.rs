use crate::commands::{get_absolute_path, traverse_back, traverse_home};
use chrono::{DateTime, Local};
use std::fs::DirEntry;
use std::os::unix::fs::MetadataExt;
use std::os::unix::prelude::PermissionsExt;
use std::{fs, io};
use termion::{color, style};
use users::{get_group_by_gid, get_user_by_uid};

fn parse_flags(input: &str, flag_a: &mut bool, flag_f: &mut bool, flag_l: &mut bool) -> String {
    let mut modified_input = input.to_string();
    let mut flag_buffer = String::new();
    let mut all_flags = Vec::new();
    let mut parsing_flags = false;

    for ch in modified_input.chars() {
        if parsing_flags {
            if ch.is_whitespace() {
                // Stop searching for flags on whitespace
                parsing_flags = false;
                process_flag(&mut flag_buffer, flag_a, flag_f, flag_l);
                all_flags.push(flag_buffer.clone());
            } else {
                // Continue building the flag buffer
                flag_buffer.push(ch);
            }
        } else if ch == '-' {
            // Start searching for flags on encountering '-'
            parsing_flags = true;
            flag_buffer.clear();
        }
    }
    all_flags.push(flag_buffer.clone());

    // Process any remaining flags after the loop
    process_flag(&mut flag_buffer, flag_a, flag_f, flag_l);

    // Remove recognized flags from the input
    for flag in &all_flags {
        modified_input = modified_input
            .replace(&format!("-{flag}"), "")
            .trim()
            .to_string();
    }

    modified_input.trim().to_string()
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

fn list_files_l(entry: &DirEntry) -> io::Result<()> {
    let metadata = entry.metadata()?;
    let file_name = entry.file_name();
    let file_type = entry.file_type()?;

    let permissions = get_permissions_string(metadata.permissions().mode());
    let owner_uid = metadata.uid();
    let owner_name = get_username(owner_uid);
    let group_uid = metadata.gid();
    let group_name = get_groupname(group_uid);
    let size = metadata.len();
    let modification_time = DateTime::<Local>::from(metadata.modified()?);

    print!(
        "{:<10} {:>10} {:>10} {:>8} {:>12} ",
        permissions,
        owner_name,
        group_name,
        size,
        modification_time.format("%b %e %H:%M"),
    );

    if file_type.is_dir() {
        print!("{}", style::Bold);
        print!("{}", color::Fg(color::Cyan));
    }

    print!("{}", file_name.to_string_lossy());
    print!("{}", color::Fg(color::Reset));
    print!("{}", style::Reset);
    Ok(())
}

fn get_permissions_string(mode: u32) -> String {
    let mut permissions = String::new();

    // File type
    if mode & 0o170000 == 0o100000 {
        permissions.push('-');
    } else if mode & 0o170000 == 0o040000 {
        permissions.push('d');
    } else {
        permissions.push('?'); // Unknown type
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
pub fn ls(args: String) {
    let mut flag_a = false;
    let mut flag_f = false;
    let mut flag_l = false;
    let input = parse_flags(&args, &mut flag_a, &mut flag_f, &mut flag_l);
    let mut path = format!("{}/{}", get_absolute_path(), input);

    if input.starts_with("..") {
        path = traverse_back(&input);
    }

    if input.starts_with('~') {
        path = traverse_home(&input);
    }

    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("ls: {e}: {input}");
            return;
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
        println!("total {total}");
        for entry in &sorted_entries {
            list_files_l(entry).unwrap();

            let file_type = entry.file_type().unwrap();
            if file_type.is_dir() && flag_f {
                print!("/")
            }
            println!();
        }
        return;
    }

    for entry in sorted_entries {
        let file_name = entry.file_name().to_string_lossy().to_string();
        let file_type = entry.file_type().unwrap();
        if file_type.is_file() {
            print!("{} ", file_name);
        } else {
            print!("{}", style::Bold);
            print!("{}", color::Fg(color::Cyan));
            print!("{}", file_name);
            print!("{}", color::Fg(color::Reset));
            print!("{}", style::Reset);
            if flag_f {
                print!("/")
            }
            print!("\t");
        }
    }
    println!();
}
