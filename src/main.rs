use swayipc::{Connection, EventType, Node, Workspace};
use swayipc::{Event, NodeType, WindowChange};
mod models;
use self::models::i3_info::I3Info;

fn rename_workspaces(mut conn: Connection) {
    // Check if focused workspace is in "allowed list".
    // If `workspaces` is empty, skip allow all workspaces.
    // let workspaces = conn.get_workspaces().unwrap();
    // println!("{:?}", workspaces);

    let i3_info = I3Info::new(conn);

    // {
        for node in i3_info.get_leaves() {
            println!("Leaves: {:?}", node);
        }
    // }
    // let parent_child = i3_info.dfs_parent_child();
    // for node in parent_child.keys() {
    //     println!("Parent-Child: {:?} {:?}", node,  parent_child.get(node));
    // }
}

fn process_node(node: Node, workspaces: Vec<Workspace>) {}

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

fn on_exit(conn: &mut Connection) {
    let workspaces = conn.get_workspaces().unwrap();
    if !workspaces.is_empty() {
        for workspace in workspaces {}
    }
}
