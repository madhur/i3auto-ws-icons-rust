use swayipc::{Connection, EventType};
use swayipc::{Event, WindowChange};
mod models;
use self::models::i3_info::I3Info;

fn rename_workspaces(conn: Connection) {
    // Check if focused workspace is in "allowed list".
    // If `workspaces` is empty, skip allow all workspaces.
    // let workspaces = conn.get_workspaces().unwrap();
    // println!("{:?}", workspaces);

    let i3_info = I3Info::new(conn);

    let _leaves = i3_info.get_leaves();
    let _parent_child = i3_info.dfs_parent_child();
    
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
                //println!("{:?}", e);
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}

