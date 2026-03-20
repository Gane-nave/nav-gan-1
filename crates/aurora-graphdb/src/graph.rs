//! Core graph data structure — adjacency list with typed nodes and edges.

use std::collections::HashMap;

/// Unique identifier for a graph node.
pub type NodeId = u64;

/// Unique identifier for a graph edge.
pub type EdgeId = u64;

/// A node in the navigation graph.
#[derive(Debug, Clone)]
pub struct GraphNode {
    /// Node identifier.
    pub id: NodeId,
    /// Latitude in degrees.
    pub lat: f64,
    /// Longitude in degrees.
    pub lon: f64,
    /// Optional label (e.g., intersection name).
    pub label: Option<String>,
    /// Arbitrary key-value properties.
    pub properties: HashMap<String, String>,
}

impl GraphNode {
    /// Create a new node at a geographic position.
    pub fn new(id: NodeId, lat: f64, lon: f64) -> Self {
        Self {
            id,
            lat,
            lon,
            label: None,
            properties: HashMap::new(),
        }
    }

    /// Set the label.
    pub fn with_label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }

    /// Add a property.
    pub fn with_property(mut self, key: &str, value: &str) -> Self {
        self.properties.insert(key.to_string(), value.to_string());
        self
    }
}

/// An edge connecting two nodes.
#[derive(Debug, Clone)]
pub struct GraphEdge {
    /// Edge identifier.
    pub id: EdgeId,
    /// Source node.
    pub from: NodeId,
    /// Target node.
    pub to: NodeId,
    /// Edge weight (e.g., distance in meters or travel time in seconds).
    pub weight: f64,
    /// Whether the edge is bidirectional.
    pub bidirectional: bool,
    /// Edge type classification.
    pub edge_type: EdgeType,
    /// Arbitrary key-value properties.
    pub properties: HashMap<String, String>,
}

/// Classification of edge types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EdgeType {
    /// Standard road segment.
    Road,
    /// Highway / motorway.
    Highway,
    /// Pedestrian path.
    Pedestrian,
    /// Bicycle lane.
    Bicycle,
    /// Public transit link.
    Transit,
    /// Ferry or water crossing.
    Ferry,
    /// Custom type.
    Custom,
}

impl GraphEdge {
    /// Create a new directed edge.
    pub fn new(id: EdgeId, from: NodeId, to: NodeId, weight: f64) -> Self {
        Self {
            id,
            from,
            to,
            weight,
            bidirectional: false,
            edge_type: EdgeType::Road,
            properties: HashMap::new(),
        }
    }

    /// Make the edge bidirectional.
    pub fn bidirectional(mut self) -> Self {
        self.bidirectional = true;
        self
    }

    /// Set the edge type.
    pub fn with_type(mut self, edge_type: EdgeType) -> Self {
        self.edge_type = edge_type;
        self
    }
}

/// Navigation graph — adjacency list representation.
pub struct NavGraph {
    nodes: HashMap<NodeId, GraphNode>,
    /// Outgoing edges per node: node_id -> `Vec<GraphEdge>`
    adjacency: HashMap<NodeId, Vec<GraphEdge>>,
    /// Reverse adjacency for incoming edges.
    reverse_adjacency: HashMap<NodeId, Vec<EdgeId>>,
    /// All edges by ID.
    edges: HashMap<EdgeId, GraphEdge>,
    next_edge_id: EdgeId,
}

impl NavGraph {
    /// Create an empty graph.
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            adjacency: HashMap::new(),
            reverse_adjacency: HashMap::new(),
            edges: HashMap::new(),
            next_edge_id: 1,
        }
    }

    /// Add a node to the graph.
    pub fn add_node(&mut self, node: GraphNode) -> NodeId {
        let id = node.id;
        self.nodes.insert(id, node);
        self.adjacency.entry(id).or_default();
        id
    }

    /// Add an edge. Returns the edge ID.
    pub fn add_edge(&mut self, mut edge: GraphEdge) -> EdgeId {
        let eid = self.next_edge_id;
        self.next_edge_id += 1;
        edge.id = eid;

        let from = edge.from;
        let to = edge.to;
        let bidi = edge.bidirectional;

        self.edges.insert(eid, edge.clone());
        self.adjacency.entry(from).or_default().push(edge.clone());
        self.reverse_adjacency.entry(to).or_default().push(eid);

        if bidi {
            let rev_eid = self.next_edge_id;
            self.next_edge_id += 1;
            let reverse = GraphEdge {
                id: rev_eid,
                from: to,
                to: from,
                weight: edge.weight,
                bidirectional: true,
                edge_type: edge.edge_type,
                properties: edge.properties,
            };
            self.edges.insert(rev_eid, reverse.clone());
            self.adjacency.entry(to).or_default().push(reverse);
            self.reverse_adjacency
                .entry(from)
                .or_default()
                .push(rev_eid);
        }

        eid
    }

    /// Get a node by ID.
    pub fn get_node(&self, id: NodeId) -> Option<&GraphNode> {
        self.nodes.get(&id)
    }

    /// Get an edge by ID.
    pub fn get_edge(&self, id: EdgeId) -> Option<&GraphEdge> {
        self.edges.get(&id)
    }

    /// Get outgoing edges from a node.
    pub fn outgoing_edges(&self, node: NodeId) -> &[GraphEdge] {
        self.adjacency.get(&node).map_or(&[], |v| v.as_slice())
    }

    /// Get neighbors (outgoing) of a node.
    pub fn neighbors(&self, node: NodeId) -> Vec<NodeId> {
        self.outgoing_edges(node).iter().map(|e| e.to).collect()
    }

    /// Get incoming neighbors of a node (nodes with edges pointing to this node).
    pub fn incoming_neighbors(&self, node: NodeId) -> Vec<NodeId> {
        self.reverse_adjacency
            .get(&node)
            .map(|edge_ids| {
                edge_ids
                    .iter()
                    .filter_map(|eid| self.edges.get(eid).map(|e| e.from))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Number of nodes.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Number of edges (stored, including reverse for bidirectional).
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// All node IDs.
    pub fn node_ids(&self) -> Vec<NodeId> {
        self.nodes.keys().copied().collect()
    }

    /// Check if a node exists.
    pub fn has_node(&self, id: NodeId) -> bool {
        self.nodes.contains_key(&id)
    }

    /// Check if an edge exists between two nodes.
    pub fn has_edge(&self, from: NodeId, to: NodeId) -> bool {
        self.outgoing_edges(from).iter().any(|e| e.to == to)
    }

    /// Remove a node and all its incident edges.
    pub fn remove_node(&mut self, id: NodeId) -> bool {
        if self.nodes.remove(&id).is_none() {
            return false;
        }
        // Remove outgoing edges
        if let Some(outgoing) = self.adjacency.remove(&id) {
            for e in &outgoing {
                self.edges.remove(&e.id);
                if let Some(rev) = self.reverse_adjacency.get_mut(&e.to) {
                    rev.retain(|eid| *eid != e.id);
                }
            }
        }
        // Remove incoming edges
        if let Some(incoming_ids) = self.reverse_adjacency.remove(&id) {
            for eid in incoming_ids {
                self.edges.remove(&eid);
                for adj in self.adjacency.values_mut() {
                    adj.retain(|e| e.id != eid);
                }
            }
        }
        true
    }

    /// Degree (number of outgoing edges) for a node.
    pub fn degree(&self, node: NodeId) -> usize {
        self.outgoing_edges(node).len()
    }

    /// All nodes as an iterator.
    pub fn nodes(&self) -> impl Iterator<Item = &GraphNode> {
        self.nodes.values()
    }
}

impl Default for NavGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_graph() -> NavGraph {
        let mut g = NavGraph::new();
        g.add_node(GraphNode::new(1, 32.0, 34.0));
        g.add_node(GraphNode::new(2, 32.1, 34.1));
        g.add_node(GraphNode::new(3, 32.2, 34.2));
        g.add_edge(GraphEdge::new(0, 1, 2, 10.0));
        g.add_edge(GraphEdge::new(0, 2, 3, 15.0));
        g.add_edge(GraphEdge::new(0, 1, 3, 30.0));
        g
    }

    #[test]
    fn test_add_nodes_and_edges() {
        let g = sample_graph();
        assert_eq!(g.node_count(), 3);
        assert_eq!(g.edge_count(), 3);
    }

    #[test]
    fn test_neighbors() {
        let g = sample_graph();
        let mut nbrs = g.neighbors(1);
        nbrs.sort();
        assert_eq!(nbrs, vec![2, 3]);
    }

    #[test]
    fn test_has_edge() {
        let g = sample_graph();
        assert!(g.has_edge(1, 2));
        assert!(!g.has_edge(2, 1)); // directed
    }

    #[test]
    fn test_bidirectional_edge() {
        let mut g = NavGraph::new();
        g.add_node(GraphNode::new(1, 0.0, 0.0));
        g.add_node(GraphNode::new(2, 1.0, 1.0));
        g.add_edge(GraphEdge::new(0, 1, 2, 5.0).bidirectional());
        assert!(g.has_edge(1, 2));
        assert!(g.has_edge(2, 1));
    }

    #[test]
    fn test_remove_node() {
        let mut g = sample_graph();
        assert!(g.remove_node(2));
        assert_eq!(g.node_count(), 2);
        assert!(!g.has_node(2));
    }

    #[test]
    fn test_degree() {
        let g = sample_graph();
        assert_eq!(g.degree(1), 2);
        assert_eq!(g.degree(2), 1);
        assert_eq!(g.degree(3), 0);
    }

    #[test]
    fn test_node_properties() {
        let mut g = NavGraph::new();
        let node = GraphNode::new(1, 32.0, 34.0)
            .with_label("Main Junction")
            .with_property("speed_limit", "60");
        g.add_node(node);
        let n = g.get_node(1).unwrap();
        assert_eq!(n.label.as_deref(), Some("Main Junction"));
        assert_eq!(n.properties.get("speed_limit").unwrap(), "60");
    }

    #[test]
    fn test_edge_type() {
        let mut g = NavGraph::new();
        g.add_node(GraphNode::new(1, 0.0, 0.0));
        g.add_node(GraphNode::new(2, 1.0, 1.0));
        g.add_edge(GraphEdge::new(0, 1, 2, 5.0).with_type(EdgeType::Highway));
        let edges = g.outgoing_edges(1);
        assert_eq!(edges[0].edge_type, EdgeType::Highway);
    }

    #[test]
    fn test_empty_graph() {
        let g = NavGraph::new();
        assert_eq!(g.node_count(), 0);
        assert_eq!(g.edge_count(), 0);
        assert!(g.node_ids().is_empty());
    }

    #[test]
    fn test_incoming_neighbors_bidirectional() {
        // Regression: bidirectional edge A->B should make incoming_neighbors(A) return B
        let mut g = NavGraph::new();
        g.add_node(GraphNode::new(1, 0.0, 0.0));
        g.add_node(GraphNode::new(2, 1.0, 1.0));
        g.add_edge(GraphEdge::new(0, 1, 2, 5.0).bidirectional());

        let mut inc_1 = g.incoming_neighbors(1);
        inc_1.sort();
        assert_eq!(
            inc_1,
            vec![2],
            "incoming_neighbors(1) should be [2] for bidi edge 1<->2"
        );

        let mut inc_2 = g.incoming_neighbors(2);
        inc_2.sort();
        assert_eq!(
            inc_2,
            vec![1],
            "incoming_neighbors(2) should be [1] for bidi edge 1<->2"
        );
    }
}
