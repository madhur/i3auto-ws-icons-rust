use std::collections::{HashMap, VecDeque};
use swayipc::{Connection, NodeType, Workspace};

use super::i3_node::I3Node;
use super::i3_window::I3Window;

pub struct I3Info {
    node: I3Node,
    workspaces: Vec<Workspace>,
}

impl I3Info {
    pub fn new(mut conn: Connection) -> Self {
        let workspaces: Vec<Workspace> = conn.get_workspaces().unwrap();

        let node = conn.get_tree().unwrap();
        let i3_node = I3Node::new(node);
        return I3Info {
            workspaces: workspaces,
            node: i3_node,
        };
    }

    fn bfs_parent_child(self) -> HashMap<String, I3Window> {
        let mut parent_child: HashMap<String, I3Window> = HashMap::new();
        let mut q: VecDeque<I3Node> = VecDeque::new();
        q.push_back(self.node);

        while let Some(t) = q.pop_front() {
            let node_type = t.node_type;
            for child in t.nodes {
                if node_type == NodeType::Workspace {
                    parent_child.insert(
                        t.name.clone(),
                        I3Window {
                            id: child.id,
                            name: child.name.clone(),
                            node_type: child.node_type,
                        },
                    );
                }
                q.push_back(child);
            }
        }

        return parent_child;
    }

    fn bfs_all_children(self) -> Vec<I3Window> {
        let mut q: VecDeque<I3Node> = VecDeque::new();
        let mut nodes: Vec<I3Window> = Vec::new();
        q.push_back(self.node);

        while let Some(t) = q.pop_front() {
            nodes.push(I3Window {
                id: t.id,
                name: t.name,
                node_type: t.node_type,
            });
            for child in t.nodes {
                q.push_back(child);
            }
        }

        return nodes;
    }

    pub fn get_leaves(self) -> Vec<I3Window> {
        let mut leaves: Vec<I3Window> = Vec::new();
        for leave in self.bfs_all_children() {
            if leave.node_type == NodeType::Con {
                leaves.push(leave);
            }
        }
        return leaves;
    }
}
