use std::collections::{HashMap, VecDeque};
use swayipc::{Connection, NodeType, Workspace};
use super::i3_node::I3Node;

#[derive(Debug, Clone)]
pub struct I3Info {
    node: I3Node,
    workspaces: Vec<Workspace>,
}

impl I3Info {
    pub fn get_workspaces(self) -> Vec<Workspace> {
        return self.workspaces;
    }

    pub fn new(conn: &mut Connection) -> Self {
        let workspaces: Vec<Workspace> = conn.get_workspaces().unwrap();

        let node = conn.get_tree().unwrap();
        let i3_node = I3Node::new(node);
        return I3Info {
            workspaces: workspaces,
            node: i3_node,
        };
    }

    pub fn dfs_parent_child(&self) -> HashMap<i64, Vec<I3Node>> {
        let mut parent_child: HashMap<i64, Vec<I3Node>> = HashMap::new();
        let mut q: VecDeque<&I3Node> = VecDeque::new();
        q.push_back(&self.node);
        let mut parent_workspace = 0;

        while let Some(t) = q.pop_front() {
            let node_type = t.node_type;
            for child in &t.nodes {

                if node_type == NodeType::Workspace {
                    parent_workspace = t.id;
                }
                else if node_type == NodeType::Dockarea {
                    // We don't want to handle dock area
                    continue;
                }
                if (child.node_type == NodeType::Con || child.node_type == NodeType::FloatingCon) && child.nodes.len() == 0 {
                    let mut window_list: Vec<I3Node>;

                    let window_list_wrap = parent_child.get(&parent_workspace);
                    if window_list_wrap.is_some() {
                        window_list = window_list_wrap.unwrap().to_vec();
                    } else {
                        window_list = Vec::new();
                    }
                    window_list.push(child.to_owned());
                    parent_child.insert(parent_workspace, window_list);
                }
                q.push_front(&child);
            }
        }

        return parent_child;
    }


    fn _bfs_all_children(&self) -> Vec<I3Node> {
        let mut q: VecDeque<&I3Node> = VecDeque::new();
        let mut nodes: Vec<I3Node> = Vec::new();
        q.push_front(&self.node);

        while let Some(t) = q.pop_front() {
            if t.node_type == NodeType::Dockarea {
                continue;
            }
            let child_nodes = &t.nodes;
            let length = child_nodes.len();
            for child in child_nodes {
                q.push_front(&child);
            }
            if length == 0 {
                nodes.push(t.to_owned());
            }
        }

        return nodes;
    }

    pub fn _get_leaves(&self) -> Vec<I3Node> {
        let mut leaves: Vec<I3Node> = Vec::new();
        for leave in self._bfs_all_children() {
            if leave.node_type == NodeType::Con {
                leaves.push(leave);
            }
        }
        return leaves;
    }
}
