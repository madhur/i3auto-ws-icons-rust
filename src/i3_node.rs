use swayipc::{Connection,  Node, NodeType};
use std::collections::VecDeque;

#[derive(Debug)]
pub struct I3Node {
    conn: Connection,
    name: Option<String>,
    pub node_type: NodeType,
    node: Node,
    parent: Option<Box<I3Node>>,
    nodes: Vec<I3Node>,
    floating_nodes: Vec<I3Node>,
}

impl I3Node {
    
    pub fn new(node: Node, parent: Option<Box<I3Node>>, conn: Connection) -> Self {        
        let nodes: Vec<I3Node> = Vec::new();
        let floating_nodes: Vec<I3Node> = Vec::new();
        let node_type = node.node_type;
        let name = node.name;
       
        return I3Node { 
            conn: conn, 
            name:name, 
            node_type: node_type, 
            node: node, 
            parent: parent, 
            nodes: nodes, 
            floating_nodes: floating_nodes
        }
    }

    pub fn set_children(& mut self, conn: Connection) -> &mut Self {
         for child_node in self.node.nodes {
            self.nodes.push(I3Node::new(child_node, Some(Box::new(self)) , conn));
        }
        for child_node in self.node.floating_nodes {
            self.nodes.push(I3Node::new(child_node, Some(Box::new(self)), conn));
        }
        return self;
    }


    fn workspaces(self) -> Vec<I3Node> {
        let  workspaces: Vec<I3Node> = Vec::new();
        self.collect_workspaces(workspaces);
        return workspaces;
    }

    fn collect_workspaces(self, workspaces: Vec<I3Node>) {
        // TODO: changes with starts with
        if self.node_type == NodeType::Workspace && !self.name.unwrap().contains("__") {
            workspaces.push(self);
            return;
        }

        for node in self.node.nodes {
            self.collect_workspaces(workspaces);
        }
    }

    fn workspace(self) -> Option<I3Node> {
        if self.node_type == NodeType::Workspace {
            return Some(self);
        }

        let mut ret = self.parent;
        let i3Node: Option<I3Node>;

        while ret.is_some() {
            let boxed_node = ret.unwrap();
            let node = *boxed_node;
            if node.node_type == NodeType::Workspace {
                i3Node = Some(node);
                break;
            }
            ret = node.parent;
        }

        return i3Node;
    }

    pub fn depth_first_search(self) -> Vec<I3Node> {
        let mut history: Vec<I3Node> = Vec::new();
        let mut queue: VecDeque<I3Node> = VecDeque::from_iter(self.nodes);
        queue.append(&mut VecDeque::from_iter(self.floating_nodes));

        while !queue.is_empty() {
            let con = queue.pop_back();

            if con.is_some() {
                history.push(con.unwrap());
            }

            queue.append(&mut VecDeque::from_iter(self.nodes));
            queue.append(&mut VecDeque::from_iter(self.floating_nodes));
        }

        // If all vertex is visited and the objective is not found
        // return a Optional with None value
        return history;
    }

    pub fn get_leaves(self) -> Vec<I3Node> {
        let mut leaves: Vec<I3Node> = Vec::new();
        for leave in self.depth_first_search() {
            if leave.node_type == NodeType::Con {
                leaves.push(leave);
            }
        }
        return leaves;
    }
}
