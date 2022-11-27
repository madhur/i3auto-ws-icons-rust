use swayipc::NodeType;

use super::i3_node::I3Node;

#[derive(Debug, Clone)]
pub struct I3Window {
    pub id: i64,
    pub name: String,
    pub node_type: NodeType,
}

