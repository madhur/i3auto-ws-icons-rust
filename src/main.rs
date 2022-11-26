use swayipc::{Connection, EventType, Node, Workspace};
use swayipc::{Event, NodeType, WindowChange};
mod i3_node;
use i3_node::I3Node;

fn rename_workspaces(mut conn: Connection) {
    // Check if focused workspace is in "allowed list".
    // If `workspaces` is empty, skip allow all workspaces.
    let workspaces = conn.get_workspaces().unwrap();
    println!("{:?}", workspaces);

    let i3_node = I3Node::new(conn.get_tree().unwrap(), None, conn).set_children(conn);

    for node in i3_node.get_leaves() {
        println!("{:?}", node);

        if node.node_type == NodeType::Dockarea {
            // ignore Nodes of type dockArea
            continue;
        } else if node.node_type == NodeType::Workspace {
        }
    }
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
                    rename_workspaces(conn)
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
