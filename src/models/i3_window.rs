use swayipc::NodeType;

#[derive(Debug, Clone)]
pub struct I3Window {
    pub id: i64,
    pub name: String,
    pub node_type: NodeType,
}