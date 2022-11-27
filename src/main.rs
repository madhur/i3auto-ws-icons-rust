use models::i3_node::I3Node;
use std::collections::HashMap;
use swayipc::{Connection, EventType};
use swayipc::{Event, WindowChange};
mod models;
use self::models::i3_info::I3Info;
use crate::models::name_parts::NameParts;
mod util;
use self::models::config::Config;
use std::time::Instant;

fn rename_workspaces(conn: &mut Connection, fa_map: &mut HashMap<String, String>) {
    let now = Instant::now();
    let i3_info = I3Info::new(conn);
    let config = read_config();
    let parent_child = i3_info.dfs_parent_child();
    // print_info(&parent_child);
    let workspaces = i3_info.get_workspaces();
    // println!("workspaces {:?}", workspaces);

    for workspace in workspaces {
        // get leaves of workspace
        let leaves_wrap = parent_child.get(&workspace.id);
        if let Some(leaves) = leaves_wrap {
            let workspace_name = workspace.name.as_str();
            let name_parts = util::parse_workspace_name(workspace_name.to_string());
            // println!("{:?}", name_parts);
            let mut icon_list = Vec::new();
            for leaf in leaves {
                // println!("Inspect leaf node: {:?}", leaf);
                icon_list.push(util::icon_for_window(
                    &leaf.window_id,
                    config.as_ref().unwrap(),
                    fa_map.to_owned(),
                ));
            }
            let formatted_icon_list = util::format_icon_list(icon_list);

            let new_workspace_name;
            if let Some(name_part) = name_parts {
                new_workspace_name = util::construct_workspace_name(NameParts {
                    num: name_part.num,
                    short_name: name_part.short_name,
                    icon: formatted_icon_list,
                });
            } else {
                new_workspace_name = util::construct_workspace_name(NameParts {
                    num: workspace.num,
                    short_name: String::from(""),
                    icon: formatted_icon_list,
                });
            }
            conn.run_command(format!(
                "rename workspace \"{}\" to \"{}\"",
                workspace_name, new_workspace_name
            ));
        }
    }
    let elapsed = now.elapsed();
    println!("Completed rename loop in : {:.2?}", elapsed);
}

fn main() -> Result<(), std::io::Error> {
    let mut conn = Connection::new().unwrap();
    let mut font_awesome = util::read_font_awesome();
    rename_workspaces(&mut conn, &mut font_awesome);
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
                    rename_workspaces(&mut Connection::new().unwrap(), &mut font_awesome);
                }
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}

fn print_info(parent_child: &HashMap<i64, Vec<I3Node>>) {
    for parent in parent_child.keys() {
        println!(
            "Workspace: Child {:?} -> {:?}",
            parent,
            parent_child.get(parent)
        );
    }
}

fn read_config() -> Option<Config> {
    // Variable that holds the filename as a `&str`.
    let config_path = util::find_file("config.toml", None, Some("toml"));
    //println!("config_path {:?}", config_path);
    if let Some(path) = config_path {
        let config: Config = util::deserialize_toml_file(path.as_path());
        return Some(config);
    }
    return None;
}
