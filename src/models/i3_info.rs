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

    pub fn dfs_parent_child(self) -> HashMap<i64, Vec<I3Window>> {
        let mut parent_child: HashMap<i64, Vec<I3Window>> = HashMap::new();
        let mut q: VecDeque<I3Node> = VecDeque::new();
        q.push_back(self.node);
        let mut parent_workspace = 0;

        while let Some(t) = q.pop_front() {
            let node_type = t.node_type;
            for child in t.nodes {
                if node_type == NodeType::Workspace {
                    parent_workspace = t.id;
                }
                if child.node_type == NodeType::Con || child.node_type == NodeType::FloatingCon {
                    let mut window_list: Vec<I3Window>;

                    let window_list_wrap = parent_child.get(&parent_workspace);
                    if window_list_wrap.is_some() {
                        window_list = window_list_wrap.unwrap().to_vec();
                    } else {
                        window_list = Vec::new();
                    }
                    window_list.push(I3Window {
                        id: child.id,
                        name: child.name.clone(),
                        node_type: child.node_type,
                    });
                    parent_child.insert(parent_workspace, window_list);
                }
                q.push_front(child);
            }
        }

        return parent_child;
    }

    pub fn bfs_parent_child_old(self) -> HashMap<i64, Vec<I3Window>> {
        let mut parent_child: HashMap<i64, Vec<I3Window>> = HashMap::new();
        let mut q: VecDeque<I3Node> = VecDeque::new();
        q.push_back(self.node);
        let mut parent_workspace = 0;

        while let Some(t) = q.pop_front() {
            let node_type = t.node_type;
            for child in t.nodes {
                if node_type == NodeType::Workspace {
                    parent_workspace = t.id;
                }
                if child.node_type == NodeType::Con || child.node_type == NodeType::FloatingCon {
                    let mut window_list: Vec<I3Window>;

                    let window_list_wrap = parent_child.get(&parent_workspace);
                    if window_list_wrap.is_some() {
                        window_list = window_list_wrap.unwrap().to_vec();
                        window_list.push(I3Window {
                            id: child.id,
                            name: child.name.clone(),
                            node_type: child.node_type,
                        });
                    } else {
                        window_list = Vec::new();
                    }
                    parent_child.insert(parent_workspace, window_list);
                }
                q.push_back(child);
            }
        }

        return parent_child;
    }

    fn bfs_all_children(self) -> Vec<I3Window> {
        let mut q: VecDeque<I3Node> = VecDeque::new();
        let mut nodes: Vec<I3Window> = Vec::new();
        q.push_front(self.node);

        while let Some(t) = q.pop_front() {
            if t.node_type == NodeType::Dockarea {
                continue;
            }
            let child_nodes = t.nodes;
            let length = child_nodes.len();
            for child in child_nodes {
                q.push_front(child);
            }
            if length == 0 {
                nodes.push(I3Window {
                    id: t.id,
                    name: t.name,
                    node_type: t.node_type,
                });
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
