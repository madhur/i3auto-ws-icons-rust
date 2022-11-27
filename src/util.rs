use super::models::config::Config;
use super::models::name_parts::NameParts;
use dirs_next::{config_dir, data_dir};
use regex::Regex;
use serde::de::DeserializeOwned;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use xcb::xproto;
use xcb::Connection;

pub fn parse_workspace_name(name: String) -> Option<NameParts> {
    let pattern = Regex::new(r"(\d+):?(\w+)? ?(.+)?").expect("Invalid regex");
    let captures = pattern.captures(&name);
    if let Some(caps) = captures {
        let num = caps.get(1).map_or("", |m| m.as_str());
        let short_name = caps.get(2).map_or("", |m| m.as_str());
        let icon = caps.get(3).map_or("", |m| m.as_str());
        return Some(NameParts {
            num: num.parse().unwrap(),
            short_name: short_name.to_string(),
            icon: icon.to_string(),
        });
    } else {
        println!("No capture for {}", name);
    }

    return None;
}

pub fn construct_workspace_name(parts: NameParts) -> String {
    return format!("{}:{}{}", parts.num, parts.short_name, parts.icon);
}

pub fn class_for_window(window: i32) -> String {
    let (conn, screen_num) = xcb::Connection::connect(None).unwrap();
    return get_class(&conn, &window);
}

pub fn format_icon_list(icon_list: Vec<String>) -> String {
    return icon_list.join(" ");
}

fn get_class(conn: &Connection, id: &i32) -> String {
    let window: xproto::Window = *id as u32;
    let long_length: u32 = 8;
    let mut long_offset: u32 = 0;
    let mut buf = Vec::new();
    loop {
        let cookie = xproto::get_property(
            &conn,
            false,
            window,
            xproto::ATOM_WM_CLASS,
            xproto::ATOM_STRING,
            long_offset,
            long_length,
        );
        match cookie.get_reply() {
            Ok(reply) => {
                let value: &[u8] = reply.value();
                buf.extend_from_slice(value);
                match reply.bytes_after() {
                    0 => break,
                    _ => {
                        let len = reply.value_len();
                        long_offset += len / 4;
                    }
                }
            }
            Err(err) => {
                println!("Error in getting class {:?}", err);
                break;
            }
        }
    }
    let result = String::from_utf8(buf).unwrap();
    return result;
}

pub fn get_icon_from_classes(config: &Config, window_classes: String) -> Option<String> {
    let results: Vec<&str> = window_classes.split('\0').collect();
    for str in results {
        println!("checking icon for {}", str);
        let window_icon = config.icons.icons.get(&str.to_lowercase());
        if let Some(icon) = window_icon {
            return Some(icon.to_string());
        }
    }
    return None;
}

pub fn find_file(file: &str, subdir: Option<&str>, extension: Option<&str>) -> Option<PathBuf> {
    // Set (or update) the extension
    let mut file = PathBuf::from(file);
    if let Some(extension) = extension {
        file.set_extension(extension);
    }

    // Try full path
    if file.exists() {
        return Some(file);
    }

    // Try XDG_CONFIG_HOME (e.g. `~/.config`)
    if let Some(mut xdg_config) = config_dir() {
        xdg_config.push("i3auto-ws-icons-rs");
        if let Some(subdir) = subdir {
            xdg_config.push(subdir);
        }
        xdg_config.push(&file);
        if xdg_config.exists() {
            return Some(xdg_config);
        }
    }

    // Try XDG_DATA_HOME (e.g. `~/.local/share/`)
    if let Some(mut xdg_data) = data_dir() {
        xdg_data.push("i3auto-ws-icons-rs");
        if let Some(subdir) = subdir {
            xdg_data.push(subdir);
        }
        xdg_data.push(&file);
        if xdg_data.exists() {
            return Some(xdg_data);
        }
    }

    // Try `/usr/share/`
    let mut usr_share_path = PathBuf::from("/usr/share/i3auto-ws-icons-rs");
    if let Some(subdir) = subdir {
        usr_share_path.push(subdir);
    }
    usr_share_path.push(&file);
    if usr_share_path.exists() {
        return Some(usr_share_path);
    }

    None
}

pub fn deserialize_toml_file(path: &Path) -> Config {
    let result = fs::read_to_string(&path);
    let result: Result<Config, toml::de::Error> = toml::from_str(&result.unwrap());
    return result.unwrap();
}
