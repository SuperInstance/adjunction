//! Forgetful functor implementations: strip algebraic structure.
//!
//! Forgetful functors are right adjoints to free functors. They "forget"
//! structure, mapping from a rich category to a simpler one:
//!
//! - **ForgetMonoid**: Monoid → Set (strip monoid operation, keep elements)
//! - **ForgetCategory**: Category → Graph (strip composition, keep objects/edges)

use crate::counit::{Graph, ListMonoid, PathCategory};
use serde::{Deserialize, Serialize};

/// A monoid presentation: a set of generators and a multiplication table.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonoidPresentation {
    /// The underlying set of elements.
    pub elements: Vec<i32>,
    /// Multiplication table: (a, b) → c means a·b = c
    pub multiplication: Vec<(i32, i32, i32)>,
}

impl MonoidPresentation {
    /// Create a new monoid presentation.
    pub fn new(elements: Vec<i32>, multiplication: Vec<(i32, i32, i32)>) -> Self {
        Self {
            elements,
            multiplication,
        }
    }

    /// Multiply two elements using the multiplication table.
    pub fn multiply(&self, a: i32, b: i32) -> Option<i32> {
        self.multiplication
            .iter()
            .find(|(x, y, _)| *x == a && *y == b)
            .map(|(_, _, z)| *z)
    }
}

/// The forgetful functor from Monoid to Set (concrete: ListMonoid → Vec<i32>).
///
/// Strips the monoid structure, returning just the underlying list of elements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgetMonoid;

impl ForgetMonoid {
    /// Apply the forgetful functor: monoid → underlying set.
    ///
    /// For a ListMonoid, this just extracts the element list.
    pub fn apply(m: &ListMonoid) -> Vec<i32> {
        m.elements.clone()
    }

    /// Apply to a list monoid and return the element count.
    pub fn count(m: &ListMonoid) -> usize {
        m.elements.len()
    }

    /// Check if a monoid element maps to the empty set (identity).
    pub fn is_empty(m: &ListMonoid) -> bool {
        m.elements.is_empty()
    }
}

/// The forgetful functor from Category to Graph.
///
/// Strips the composition structure, keeping only objects and generating morphisms.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgetCategory;

impl ForgetCategory {
    /// Apply the forgetful functor: PathCategory → Graph.
    ///
    /// Returns the underlying graph (vertices + edges).
    pub fn apply(cat: &PathCategory) -> Graph {
        cat.graph.clone()
    }

    /// Extract just the edge list from a category.
    pub fn edges(cat: &PathCategory) -> Vec<(usize, usize)> {
        cat.graph.edges.clone()
    }

    /// Extract just the number of objects (vertices).
    pub fn num_objects(cat: &PathCategory) -> usize {
        cat.graph.num_vertices
    }

    /// Check if the underlying graph is empty.
    pub fn is_empty(cat: &PathCategory) -> bool {
        cat.graph.num_vertices == 0 && cat.graph.edges.is_empty()
    }
}

/// Forget the group structure, returning just the generators.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgetGroup;

impl ForgetGroup {
    /// Apply: group presentation → generators as strings.
    pub fn apply(generators: &[String]) -> Vec<String> {
        generators.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forget_monoid_apply() {
        let m = ListMonoid::new(vec![1, 2, 3]);
        let set = ForgetMonoid::apply(&m);
        assert_eq!(set, vec![1, 2, 3]);
    }

    #[test]
    fn test_forget_monoid_empty() {
        let m = ListMonoid::identity();
        assert!(ForgetMonoid::is_empty(&m));
        assert_eq!(ForgetMonoid::count(&m), 0);
    }

    #[test]
    fn test_forget_monoid_count() {
        let m = ListMonoid::new(vec![1, 2]);
        assert_eq!(ForgetMonoid::count(&m), 2);
    }

    #[test]
    fn test_forget_category_apply() {
        let cat = PathCategory::from_graph(&Graph::new(3, vec![(0, 1), (1, 2)]));
        let graph = ForgetCategory::apply(&cat);
        assert_eq!(graph.num_vertices, 3);
        assert_eq!(graph.edges.len(), 2);
    }

    #[test]
    fn test_forget_category_edges() {
        let cat = PathCategory::from_graph(&Graph::new(2, vec![(0, 1)]));
        let edges = ForgetCategory::edges(&cat);
        assert_eq!(edges, vec![(0, 1)]);
    }

    #[test]
    fn test_forget_category_num_objects() {
        let cat = PathCategory::from_graph(&Graph::new(5, vec![]));
        assert_eq!(ForgetCategory::num_objects(&cat), 5);
    }



    #[test]
    fn test_forget_group_apply() {
        let gens = vec!["a".to_string(), "b".to_string()];
        let result = ForgetGroup::apply(&gens);
        assert_eq!(result, vec!["a", "b"]);
    }

    #[test]
    fn test_monoid_presentation_multiply() {
        // Z/3 under addition: 0+0=0, 0+1=1, 0+2=2, 1+0=1, 1+1=2, 1+2=0, etc.
        let pres = MonoidPresentation::new(
            vec![0, 1, 2],
            vec![
                (0, 0, 0),
                (0, 1, 1),
                (0, 2, 2),
                (1, 0, 1),
                (1, 1, 2),
                (1, 2, 0),
                (2, 0, 2),
                (2, 1, 0),
                (2, 2, 1),
            ],
        );
        assert_eq!(pres.multiply(1, 2), Some(0));
        assert_eq!(pres.multiply(2, 2), Some(1));
    }



    #[test]
    fn test_forget_category_roundtrip() {
        let original = Graph::new(4, vec![(0, 1), (1, 2), (2, 3), (3, 0)]);
        let cat = PathCategory::from_graph(&original);
        let recovered = ForgetCategory::apply(&cat);
        assert_eq!(recovered.num_vertices, original.num_vertices);
        assert_eq!(recovered.edges, original.edges);
    }
}
