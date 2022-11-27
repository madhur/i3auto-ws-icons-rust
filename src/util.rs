use super::models::config::Config;
use super::models::name_parts::NameParts;
use crate::models::font_awesome::FAConfig;
use dirs_next::{config_dir, data_dir};
use regex::Regex;
use std::char::from_u32;
use std::collections::HashMap;
use std::fs;
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

fn get_icon_from_classes(config: &Config, window_classes: String) -> Option<String> {
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

pub fn class_for_window(window: i32) -> String {
    let (conn, _screen_num) = xcb::Connection::connect(None).unwrap();
    return get_class(&conn, &window);
}

pub fn format_icon_list(icon_list: Vec<String>) -> String {
    return icon_list.join(" ");
}

pub fn icon_for_window(
    window_id: &Option<i64>,
    config: &Config,
    fa_map: HashMap<String, String>,
) -> String {
    let default_icon = config.default_icon.to_owned();
    if let Some(win_id) = window_id {
        let window_classes = class_for_window(*win_id as i32);
        let window_class = get_icon_from_classes(config, window_classes);
        if let Some(class) = window_class {
            let unicode_chars = fa_map.get(&class);
            if let Some(unicode) = unicode_chars {
                let result = from_u32(u32::from_str_radix(unicode.as_str(), 16).unwrap()).unwrap();

                return result.to_string();
            }
        }
    }
    return default_icon;
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

pub fn deserialize_fa_toml_file(path: &Path) -> FAConfig {
    let result = fs::read_to_string(&path);
    let result: Result<FAConfig, toml::de::Error> = toml::from_str(&result.unwrap());
    return result.unwrap();
}

pub fn read_font_awesome() -> HashMap<String, String> {
    let file = PathBuf::from("src/assets/char_list.toml");
    println!("{:?}", file);
    let config = deserialize_fa_toml_file(&file);
    let mut fa_map: HashMap<String, String> = HashMap::new();
    for solid in config.solid {
        fa_map.insert(solid.name, solid.unicode);
    }
    for brand in config.brands {
        fa_map.insert(brand.name, brand.unicode);
    }
    return fa_map;
}
