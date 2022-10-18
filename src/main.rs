use swayipc::{Connection, EventType, Workspace};
use swayipc::{Event, NodeLayout, NodeType, WindowChange};

fn rename_workspaces(conn: &mut Connection) {
    // Check if focused workspace is in "allowed list".
    // If `workspaces` is empty, skip allow all workspaces.
    let workspaces =  conn.get_workspaces().unwrap();
    if !workspaces.is_empty() {
        for workspace in workspaces
        {
          
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut conn = Connection::new().unwrap();
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
                println!("{:?}", e);
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}
