use models::i3_node::I3Node;
use std::collections::HashMap;
use swayipc::{Connection, EventType};
use swayipc::{Event, WindowChange};
mod models;
use crate::models::name_parts::NameParts;

use self::models::i3_info::I3Info;
mod util;
use self::models::config::Config;

fn rename_workspaces(conn: &mut Connection) {
    let i3_info = I3Info::new(conn);
    let config = read_config();
    let parent_child = i3_info.dfs_parent_child();
    print_info(&parent_child);
    let workspaces = i3_info.get_workspaces();
    println!("workspaces {:?}", workspaces);

    for workspace in workspaces {
        // get leaves of workspace
        let leaves_wrap = parent_child.get(&workspace.id);
        if let Some(leaves) = leaves_wrap {
            let workspace_name = workspace.name.as_str();
            let name_parts = util::parse_workspace_name(workspace_name.to_string());
            println!("{:?}", name_parts);
            let mut icon_list = Vec::new();
            for leaf in leaves {
                icon_list.push(icon_for_window(&leaf.window_id, config.as_ref().unwrap()));
            }
            let formatted_icon_list = util::format_icon_list(icon_list);
            println!("{:?}", formatted_icon_list);

            let new_workspace_name;
            if let Some(name_part) = name_parts {
                new_workspace_name = util::construct_workspace_name(NameParts {
                    num: name_part.num,
                    short_name: name_part.short_name,
                    icon: formatted_icon_list,
                });
            }
            else {
                    new_workspace_name = util::construct_workspace_name(NameParts {
                    num: workspace.num,
                    short_name: String::from(""),
                    icon: formatted_icon_list,
                });
            }

            

            conn.run_command(format!("rename workspace \"{}\" to \"{}\"", workspace_name, new_workspace_name));
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut conn = Connection::new().unwrap();
    rename_workspaces(&mut conn);
    for event in Connection::new()
        .unwrap()
        .subscribe(&[EventType::Window])
        .unwrap()
    {
        match event.unwrap() {
            Event::Window(e) => {
                if WindowChange::New == e.change
                    || WindowChange::Close == e.change
                    || WindowChange::Move == e.change
                {
                    rename_workspaces(&mut Connection::new().unwrap());
                }
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}

fn print_info(parent_child: &HashMap<i64, Vec<I3Node>>) {
    for parent in parent_child.keys() {
        println!("Parent {:?} -> {:?}", parent, parent_child.get(parent));
    }
}

fn icon_for_window(window_id: &Option<i64>, config: &Config) -> String {
    let default_icon = config.default_icon.to_owned();
    if let Some(win_id) = window_id {
        let window_classes = util::class_for_window(*win_id as i32);
        let window_class = util::get_icon_from_classes(config, window_classes);
        if let Some(class) = window_class {
            return class;
        }
    }
    return default_icon;
}

fn read_config() -> Option<Config> {
    // Variable that holds the filename as a `&str`.
    let config_path = util::find_file("config.toml", None, Some("toml"));
    println!("config_path {:?}", config_path);
    if let Some(path) = config_path {
        let config: Config = util::deserialize_toml_file(path.as_path());
        return Some(config);
    }
    return None;
}
