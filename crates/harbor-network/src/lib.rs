use std::collections::{HashMap, HashSet};
use std::fmt;

use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::EdgeRef;
use petgraph::Undirected;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DemandBasis {
    PeakSeason,
    Average,
}

impl DemandBasis {
    pub fn key(self) -> &'static str {
        match self {
            Self::PeakSeason => "peak-season",
            Self::Average => "average",
        }
    }
}

impl fmt::Display for DemandBasis {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.key())
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Port {
    pub id: String,
    pub name: String,
    pub role: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Lane {
    pub id: String,
    pub capacity_teu: f64,
    pub basis: DemandBasis,
}

#[derive(Debug, Error, PartialEq)]
pub enum NetworkError {
    #[error("port id already exists: {0}")]
    DuplicatePort(String),
    #[error("lane id already exists: {0}")]
    DuplicateLane(String),
    #[error("unknown port id: {0}")]
    UnknownPort(String),
    #[error("lane capacity_teu must be positive for {lane_id}: {capacity_teu}")]
    NonPositiveCapacity { lane_id: String, capacity_teu: f64 },
}

#[derive(Debug, Default)]
pub struct Network {
    graph: Graph<Port, Lane, Undirected>,
    ports_by_id: HashMap<String, NodeIndex>,
    lane_ids: HashSet<String>,
}

impl Network {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_port(&mut self, port: Port) -> Result<(), NetworkError> {
        if self.ports_by_id.contains_key(&port.id) {
            return Err(NetworkError::DuplicatePort(port.id));
        }
        let id = port.id.clone();
        let index = self.graph.add_node(port);
        self.ports_by_id.insert(id, index);
        Ok(())
    }

    pub fn add_lane(
        &mut self,
        from_port: &str,
        to_port: &str,
        lane: Lane,
    ) -> Result<(), NetworkError> {
        if lane.capacity_teu <= 0.0 {
            return Err(NetworkError::NonPositiveCapacity {
                lane_id: lane.id,
                capacity_teu: lane.capacity_teu,
            });
        }
        if self.lane_ids.contains(&lane.id) {
            return Err(NetworkError::DuplicateLane(lane.id));
        }
        let from = self.node_index(from_port)?;
        let to = self.node_index(to_port)?;
        let lane_id = lane.id.clone();
        self.graph.add_edge(from, to, lane);
        self.lane_ids.insert(lane_id);
        Ok(())
    }

    pub fn port_count(&self) -> usize {
        self.graph.node_count()
    }

    pub fn lane_count(&self) -> usize {
        self.graph.edge_count()
    }

    pub fn degree(&self, port_id: &str) -> Result<usize, NetworkError> {
        let index = self.node_index(port_id)?;
        Ok(self.graph.edges(index).count())
    }

    pub fn incident_capacity_teu(&self, port_id: &str) -> Result<f64, NetworkError> {
        let index = self.node_index(port_id)?;
        Ok(self
            .graph
            .edges(index)
            .map(|edge| edge.weight().capacity_teu)
            .sum())
    }

    pub fn redundant_capacity_teu(&self, port_id: &str) -> Result<f64, NetworkError> {
        let index = self.node_index(port_id)?;
        let mut total = 0.0_f64;
        let mut largest = 0.0_f64;
        for edge in self.graph.edges(index) {
            let capacity = edge.weight().capacity_teu;
            total += capacity;
            largest = largest.max(capacity);
        }
        Ok(total - largest)
    }

    pub fn is_connected(&self, a: &str, b: &str) -> Result<bool, NetworkError> {
        let start = self.node_index(a)?;
        let goal = self.node_index(b)?;
        Ok(self.reachable(start, goal, None))
    }

    pub fn has_diverse_path(&self, a: &str, b: &str) -> Result<bool, NetworkError> {
        let start = self.node_index(a)?;
        let goal = self.node_index(b)?;
        if start == goal || !self.reachable(start, goal, None) {
            return Ok(false);
        }

        let mut first_hops = HashSet::new();
        for edge in self.graph.edges(start) {
            let neighbor = edge.target();
            if neighbor == goal || self.reachable(neighbor, goal, Some(start)) {
                first_hops.insert(neighbor);
            }
        }
        Ok(first_hops.len() >= 2)
    }

    fn node_index(&self, port_id: &str) -> Result<NodeIndex, NetworkError> {
        self.ports_by_id
            .get(port_id)
            .copied()
            .ok_or_else(|| NetworkError::UnknownPort(port_id.to_string()))
    }

    fn reachable(&self, start: NodeIndex, goal: NodeIndex, excluded: Option<NodeIndex>) -> bool {
        if Some(start) == excluded || Some(goal) == excluded {
            return false;
        }
        let mut visited = HashSet::new();
        let mut stack = vec![start];
        while let Some(node) = stack.pop() {
            if node == goal {
                return true;
            }
            if Some(node) == excluded || !visited.insert(node) {
                continue;
            }
            for edge in self.graph.edges(node) {
                let neighbor = edge.target();
                if Some(neighbor) != excluded && !visited.contains(&neighbor) {
                    stack.push(neighbor);
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn port(id: &str) -> Port {
        Port {
            id: id.to_string(),
            name: format!("{id} port"),
            role: "gateway".to_string(),
        }
    }

    fn lane(id: &str, capacity_teu: f64, basis: DemandBasis) -> Lane {
        Lane {
            id: id.to_string(),
            capacity_teu,
            basis,
        }
    }

    fn three_port_chain() -> Network {
        let mut network = Network::new();
        network
            .add_port(port("a"))
            .expect("port a should be accepted");
        network
            .add_port(port("b"))
            .expect("port b should be accepted");
        network
            .add_port(port("c"))
            .expect("port c should be accepted");
        network
            .add_lane("a", "b", lane("ab", 1000.0, DemandBasis::PeakSeason))
            .expect("lane ab should be accepted");
        network
            .add_lane("b", "c", lane("bc", 750.0, DemandBasis::Average))
            .expect("lane bc should be accepted");
        network
    }

    #[test]
    fn builds_graph_and_counts_ports_and_lanes() {
        let network = three_port_chain();
        assert_eq!(network.port_count(), 3);
        assert_eq!(network.lane_count(), 2);
    }

    #[test]
    fn incident_capacity_sums_lane_ratings() {
        let network = three_port_chain();
        assert_eq!(network.incident_capacity_teu("b"), Ok(1750.0));
        assert_eq!(network.incident_capacity_teu("a"), Ok(1000.0));
    }

    #[test]
    fn redundant_capacity_removes_largest_incident_lane() {
        let network = three_port_chain();
        assert_eq!(network.redundant_capacity_teu("b"), Ok(750.0));
        assert_eq!(network.redundant_capacity_teu("a"), Ok(0.0));
    }

    #[test]
    fn degree_counts_incident_lanes() {
        let network = three_port_chain();
        assert_eq!(network.degree("b"), Ok(2));
        assert_eq!(network.degree("c"), Ok(1));
    }

    #[test]
    fn connectivity_distinguishes_reachable_and_gap() {
        let mut network = three_port_chain();
        network.add_port(port("d")).expect("port d accepted");
        assert_eq!(network.is_connected("a", "c"), Ok(true));
        assert_eq!(network.is_connected("a", "d"), Ok(false));
    }

    #[test]
    fn chain_has_no_diverse_path() {
        let network = three_port_chain();
        assert_eq!(network.has_diverse_path("a", "c"), Ok(false));
    }

    #[test]
    fn ring_has_diverse_path() {
        let mut network = three_port_chain();
        network
            .add_lane("a", "c", lane("ac", 500.0, DemandBasis::PeakSeason))
            .expect("lane ac should be accepted");
        assert_eq!(network.has_diverse_path("a", "c"), Ok(true));
        assert_eq!(network.has_diverse_path("a", "b"), Ok(true));
    }

    #[test]
    fn single_direct_lane_is_not_diverse_path() {
        let mut network = Network::new();
        network.add_port(port("a")).expect("port a accepted");
        network.add_port(port("b")).expect("port b accepted");
        network
            .add_lane("a", "b", lane("ab", 500.0, DemandBasis::Average))
            .expect("lane ab accepted");
        assert_eq!(network.has_diverse_path("a", "b"), Ok(false));
    }

    #[test]
    fn lane_basis_is_preserved() {
        let network = three_port_chain();
        let bases = network
            .graph
            .edge_weights()
            .map(|lane| lane.basis)
            .collect::<HashSet<_>>();
        assert!(bases.contains(&DemandBasis::PeakSeason));
        assert!(bases.contains(&DemandBasis::Average));
    }

    #[test]
    fn duplicate_port_is_rejected_with_typed_error() {
        let mut network = Network::new();
        network
            .add_port(port("a"))
            .expect("first port should be accepted");
        assert_eq!(
            network.add_port(port("a")),
            Err(NetworkError::DuplicatePort("a".to_string()))
        );
    }

    #[test]
    fn duplicate_lane_is_rejected_with_typed_error() {
        let mut network = three_port_chain();
        assert_eq!(
            network.add_lane("a", "c", lane("ab", 250.0, DemandBasis::Average)),
            Err(NetworkError::DuplicateLane("ab".to_string()))
        );
    }

    #[test]
    fn non_positive_capacity_is_rejected_with_typed_error() {
        let mut network = Network::new();
        network
            .add_port(port("a"))
            .expect("port a should be accepted");
        network
            .add_port(port("b"))
            .expect("port b should be accepted");
        assert_eq!(
            network.add_lane("a", "b", lane("ab", 0.0, DemandBasis::Average)),
            Err(NetworkError::NonPositiveCapacity {
                lane_id: "ab".to_string(),
                capacity_teu: 0.0
            })
        );
    }

    #[test]
    fn unknown_port_is_rejected_with_typed_error() {
        let mut network = Network::new();
        network
            .add_port(port("a"))
            .expect("port a should be accepted");
        assert_eq!(
            network.add_lane("a", "missing", lane("am", 10.0, DemandBasis::Average)),
            Err(NetworkError::UnknownPort("missing".to_string()))
        );
        assert_eq!(
            network.degree("missing"),
            Err(NetworkError::UnknownPort("missing".to_string()))
        );
    }
}
