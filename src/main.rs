use std::collections::HashMap;

use models::i3_window::I3Window;
use swayipc::{Connection, EventType};
use swayipc::{Event, WindowChange};
mod models;
use self::models::i3_info::I3Info;

fn rename_workspaces(conn: Connection) {
    let i3_info = I3Info::new(conn);

    let leaves = i3_info.get_leaves();
    let parent_child = i3_info.dfs_parent_child();
    print_info(leaves, parent_child);
    let workspaces = i3_info.get_workspaces();
    println!("workspaces {:?}", workspaces);

}


fn main() -> Result<(), std::io::Error> {
    let conn = Connection::new().unwrap();
    rename_workspaces(conn);
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
                    rename_workspaces(Connection::new().unwrap());
                }                
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}

fn print_info(leaves: Vec<I3Window>, parent_child: HashMap<i64, Vec<I3Window>>) {
    for window in leaves {
         println!("Leaves {:?}", window);
    }

    for parent in parent_child.keys() {
        println!("Parent {:?} -> {:?}", parent , parent_child.get(parent));
    }

}

