
use std::{
    collections::{HashMap, VecDeque},
};
use swayipc::{Connection, NodeType, Workspace};

use super::i3_node::I3Node;

pub struct I3Info {
    node: I3Node,
    workspaces: Vec<Workspace>,
    // parent_child: HashMap<I3Node, I3Node>,
}

impl I3Info {
    pub fn new(mut conn: Connection) -> Self {
        let workspaces: Vec<Workspace> = conn.get_workspaces().unwrap();

        let node = conn.get_tree().unwrap();
        let i3Node = I3Node::new(node);
        return I3Info {
            workspaces: workspaces,
            node: i3Node,
        };
    }

    fn bfs_parent_child(self) -> HashMap<I3Node, I3Node> {
        let mut q: VecDeque<I3Node> = VecDeque::new();
        let mut nodes: Vec<I3Node> = Vec::new();
        let mut parent_child: HashMap<I3Node, I3Node> = HashMap::new();
        let parentNode = self.node;
        q.push_back(parentNode);

        while let Some(t) = q.pop_front() {
            nodes.push(t);
            if parentNode.node_type == NodeType::Workspace {
                parent_child.insert(parentNode, t);
            }
            for child in t.nodes {
                q.push_back(&child);
            }
        }

        return parent_child;
    }

    fn bfs_all_children(self) -> Vec<I3Node> {
        let mut q: VecDeque<I3Node> = VecDeque::new();
        let mut nodes: Vec<I3Node> = Vec::new();
        let parentNode = self.node;
        q.push_back(parentNode);

        while let Some(t) = q.pop_front() {
            nodes.push(t);
            for child in t.nodes {
                q.push_back(child);
            }
        }

        return nodes;
    }

    pub fn get_leaves(self) -> Vec<I3Node> {
        let mut leaves: Vec<I3Node> = Vec::new();
        for leave in self.bfs_all_children() {
            if leave.node_type == NodeType::Con {
                leaves.push(leave);
            }
        }
        return leaves;
    }
}
