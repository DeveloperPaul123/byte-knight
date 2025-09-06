/// Trait to define the node types in the search tree.
pub trait NodeType {
    const PV: bool;
    #[allow(dead_code)]
    const ROOT: bool;
}

/// The root node of the search tree.
/// This node is the starting point of the search and does not have a parent.
pub(crate) struct RootNode;

/// A node that is on the pricinple variation (PV) path.
pub(crate) struct PvNode;

/// A node that is not on the principle variation (PV) path.
pub(crate) struct NonPvNode;

impl NodeType for RootNode {
    const PV: bool = true;
    const ROOT: bool = true;
}

impl NodeType for PvNode {
    const PV: bool = true;
    const ROOT: bool = false;
}

impl NodeType for NonPvNode {
    const PV: bool = false;
    const ROOT: bool = false;
}
