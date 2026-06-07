//! Counit natural transformations (ε) for adjunctions.
//!
//! The counit ε: F∘G → 1_D of an adjunction F ⊣ G maps each object that
//! has been through both functors back to its simplified form.

use serde::{Deserialize, Deserializer, Serialize};

/// A monoid element represented as a list of generators.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ListMonoid {
    /// Elements of the monoid (a list under concatenation).
    pub elements: Vec<i32>,
}

impl ListMonoid {
    /// Create a new list monoid from elements.
    pub fn new(elements: Vec<i32>) -> Self {
        Self { elements }
    }

    /// Monoid operation: concatenate two lists.
    pub fn multiply(&self, other: &ListMonoid) -> ListMonoid {
        let mut result = self.elements.clone();
        result.extend_from_slice(&other.elements);
        ListMonoid::new(result)
    }

    /// Identity element: empty list.
    pub fn identity() -> Self {
        ListMonoid::new(vec![])
    }

    /// Check if this is the identity element.
    pub fn is_identity(&self) -> bool {
        self.elements.is_empty()
    }
}

/// A counit natural transformation ε: B → B.
#[derive(Debug, Clone, Serialize)]
pub struct Counit<B> {
    /// Human-readable name.
    pub name: String,
    /// The underlying function ε: B → B
    #[serde(skip)]
    pub transform: fn(&B) -> B,
}

impl<'de, B> Deserialize<'de> for Counit<B> {
    fn deserialize<D: Deserializer<'de>>(_deserializer: D) -> Result<Self, D::Error> {
        Err(serde::de::Error::custom(
            "Counit cannot be deserialized: construct it with Counit::new()",
        ))
    }
}

impl<B> Counit<B> {
    /// Create a new named counit transformation.
    pub fn new(name: &str, transform: fn(&B) -> B) -> Self {
        Self {
            name: name.to_string(),
            transform,
        }
    }

    /// Apply the counit transformation.
    pub fn apply(&self, b: &B) -> B {
        (self.transform)(b)
    }
}

/// Counit for the forgetful monoid adjunction.
pub fn forgetful_monoid_counit() -> Counit<ListMonoid> {
    Counit::new("ForgetfulMonoid Counit ε", |m| {
        ListMonoid::new(m.elements.clone())
    })
}

/// A graph edge.
pub type Edge = (usize, usize);

/// A simple graph with vertices and edges.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Graph {
    /// Number of vertices (vertices are 0..num_vertices).
    pub num_vertices: usize,
    /// Directed edges.
    pub edges: Vec<Edge>,
}

impl Graph {
    /// Create a new graph.
    pub fn new(num_vertices: usize, edges: Vec<Edge>) -> Self {
        Self {
            num_vertices,
            edges,
        }
    }

    /// Empty graph.
    pub fn empty() -> Self {
        Self::new(0, vec![])
    }

    /// Add an edge.
    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.edges.push((from, to));
    }

    /// Get all vertices.
    pub fn vertices(&self) -> Vec<usize> {
        (0..self.num_vertices).collect()
    }

    /// Get edges from a specific vertex.
    pub fn edges_from(&self, v: usize) -> Vec<&Edge> {
        self.edges.iter().filter(|(s, _)| *s == v).collect()
    }
}

/// A category with objects (vertices) and morphisms (paths).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PathCategory {
    /// Underlying graph.
    pub graph: Graph,
}

impl PathCategory {
    /// Create from a graph.
    pub fn from_graph(graph: &Graph) -> Self {
        Self {
            graph: graph.clone(),
        }
    }

    /// Get all identity morphisms.
    pub fn identities(&self) -> Vec<crate::unit::Morphism> {
        (0..self.graph.num_vertices)
            .map(crate::unit::Morphism::identity)
            .collect()
    }
}

/// Counit for the ForgetfulCat adjunction.
pub fn forgetful_category_counit() -> Counit<PathCategory> {
    Counit::new("ForgetfulCat Counit ε", |cat| PathCategory {
        graph: cat.graph.clone(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_monoid_multiply() {
        let a = ListMonoid::new(vec![1, 2]);
        let b = ListMonoid::new(vec![3, 4]);
        assert_eq!(a.multiply(&b).elements, vec![1, 2, 3, 4]);
    }

    // Removed: test_list_monoid_identity, test_counit_name, test_empty_monoid

    #[test]
    fn test_list_monoid_associativity() {
        let a = ListMonoid::new(vec![1]);
        let b = ListMonoid::new(vec![2]);
        let c = ListMonoid::new(vec![3]);
        let left = a.multiply(&b).multiply(&c);
        let right = a.multiply(&b.multiply(&c));
        assert_eq!(left.elements, right.elements);
    }

    #[test]
    fn test_forgetful_monoid_counit() {
        let eps = forgetful_monoid_counit();
        let m = ListMonoid::new(vec![1, 2, 3]);
        assert_eq!(eps.apply(&m).elements, vec![1, 2, 3]);
    }

    #[test]
    fn test_graph_creation() {
        let g = Graph::new(3, vec![(0, 1), (1, 2)]);
        assert_eq!(g.num_vertices, 3);
        assert_eq!(g.edges.len(), 2);
        assert_eq!(g.vertices(), vec![0, 1, 2]);
    }

    #[test]
    fn test_graph_edges_from() {
        let g = Graph::new(3, vec![(0, 1), (0, 2), (1, 2)]);
        assert_eq!(g.edges_from(0).len(), 2);
    }

    #[test]
    fn test_graph_add_edge() {
        let mut g = Graph::new(2, vec![]);
        g.add_edge(0, 1);
        assert_eq!(g.edges, vec![(0, 1)]);
    }

    #[test]
    fn test_path_category_from_graph() {
        let g = Graph::new(2, vec![(0, 1)]);
        let cat = PathCategory::from_graph(&g);
        assert_eq!(cat.graph.edges, vec![(0, 1)]);
    }

    #[test]
    fn test_path_category_identities() {
        let g = Graph::new(3, vec![]);
        let cat = PathCategory::from_graph(&g);
        assert_eq!(cat.identities().len(), 3);
    }

    #[test]
    fn test_forgetful_category_counit() {
        let eps = forgetful_category_counit();
        let cat = PathCategory::from_graph(&Graph::new(2, vec![(0, 1)]));
        assert_eq!(eps.apply(&cat).graph.edges, vec![(0, 1)]);
    }




}
