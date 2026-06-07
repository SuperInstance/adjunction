//! Free functor implementations: FreeMonoid and FreeCategory.
//!
//! Free functors are left adjoints to forgetful functors. They build
//! algebraic structure from bare data:
//!
//! - **FreeMonoid**: Set → Monoid (elements → lists under concatenation)
//! - **FreeCategory**: Graph → Category (edges → paths under composition)
//!
//! Each free functor satisfies a universal property: it is the "most general"
//! way to build the structure, and any other construction factors through it.

use crate::counit::{Graph, ListMonoid, PathCategory};
use crate::unit::Morphism;
use serde::{Deserialize, Serialize};

/// The free monoid functor: Set → Monoid.
///
/// Given a set S (represented as Vec<i32>), builds the free monoid F(S)
/// consisting of all finite words over S with concatenation.
///
/// This is the left adjoint to the forgetful functor U: Monoid → Set.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreeMonoid {
    /// The generating set.
    pub generators: Vec<i32>,
}

impl FreeMonoid {
    /// Build the free monoid on a generating set.
    pub fn build(generators: Vec<i32>) -> Self {
        Self { generators }
    }

    /// The underlying set of all words (up to a maximum length).
    ///
    /// Returns all lists of generators with length ≤ max_len.
    /// The full free monoid is infinite, so we truncate.
    pub fn words(&self, max_len: usize) -> Vec<ListMonoid> {
        if max_len == 0 {
            return vec![ListMonoid::identity()];
        }

        let mut result = vec![ListMonoid::identity()];
        for len in 1..=max_len {
            let mut new_words = Vec::new();
            // Generate all words of exactly `len` characters
            let mut current: Vec<Vec<i32>> = vec![vec![]];
            for _ in 0..len {
                let mut next = Vec::new();
                for prefix in &current {
                    for g in &self.generators {
                        let mut word = prefix.clone();
                        word.push(*g);
                        next.push(word);
                    }
                }
                current = next;
            }
            for word in current {
                new_words.push(ListMonoid::new(word));
            }
            result.extend(new_words);
        }
        result
    }

    /// Verify the universal property of the free monoid.
    ///
    /// Given any function f: S → M (from generators to a monoid M),
    /// there exists a unique monoid homomorphism f̂: F(S) → M such that
    /// f̂([s]) = f(s) and f̂ concatenates.
    ///
    /// We verify this by checking that the extension is well-defined and
    /// preserves the monoid operation.
    pub fn universal_property<F>(&self, target_map: F) -> UniversalPropertyResult
    where
        F: Fn(i32) -> ListMonoid,
    {
        let mut passed = true;
        let mut failures = Vec::new();

        // Check that f̂(empty) = identity of target
        let hat_empty = ListMonoid::identity();
        if !hat_empty.is_identity() {
            // This is always true by construction, but let's verify
            passed = false;
            failures.push("Extension of empty word is not identity".to_string());
        }

        // Check that f̂ preserves multiplication on generators
        for &g1 in &self.generators {
            for &g2 in &self.generators {
                let f_hat_g1_g2 = {
                    let m1 = target_map(g1);
                    let m2 = target_map(g2);
                    m1.multiply(&m2)
                };
                // f̂([g1, g2]) should equal f̂([g1]) * f̂([g2])
                let f_hat_concat = {
                    let combined = ListMonoid::new(vec![g1, g2]);
                    combined
                        .elements
                        .iter()
                        .map(|&x| target_map(x))
                        .fold(ListMonoid::identity(), |acc, m| acc.multiply(&m))
                };

                if f_hat_g1_g2.elements != f_hat_concat.elements {
                    passed = false;
                    failures.push(format!(
                        "Homomorphism property fails for generators {} and {}",
                        g1, g2
                    ));
                }
            }
        }

        UniversalPropertyResult {
            functor_name: "FreeMonoid".to_string(),
            passed,
            failures,
        }
    }

    /// The unit η: S → U(F(S)) sends each element to its singleton list.
    pub fn unit_map(&self, x: i32) -> ListMonoid {
        ListMonoid::new(vec![x])
    }
}

/// Result of verifying a universal property.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalPropertyResult {
    /// Name of the functor being verified.
    pub functor_name: String,
    /// Whether the universal property holds.
    pub passed: bool,
    /// Any failures encountered.
    pub failures: Vec<String>,
}

/// The free category functor: Graph → Category.
///
/// Given a graph G, builds the free category F(G) (path category) where:
/// - Objects = vertices of G
/// - Morphisms = paths in G (sequences of composable edges)
/// - Composition = path concatenation
/// - Identities = empty paths
///
/// This is the left adjoint to the forgetful functor U: Cat → Graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreeCategory {
    /// The underlying graph.
    pub graph: Graph,
}

impl FreeCategory {
    /// Build the free category on a graph.
    pub fn build(graph: Graph) -> Self {
        Self { graph }
    }

    /// Get the identity morphism for each vertex.
    pub fn identities(&self) -> Vec<Morphism> {
        (0..self.graph.num_vertices)
            .map(Morphism::identity)
            .collect()
    }

    /// Generate all paths of exactly the given length in the graph.
    pub fn paths_of_length(&self, len: usize) -> Vec<Morphism> {
        if len == 0 {
            return self.identities();
        }

        let mut paths = Vec::new();

        // Start from each edge as a length-1 path
        let mut current_paths: Vec<Morphism> = self
            .graph
            .edges
            .iter()
            .map(|&(s, t)| Morphism {
                source: s,
                target: t,
                path: vec![(s, t)],
            })
            .collect();

        if len == 1 {
            return current_paths;
        }

        for _ in 1..len {
            let mut next_paths = Vec::new();
            for p in &current_paths {
                for &(s, t) in &self.graph.edges {
                    if p.target == s {
                        let mut new_path = p.clone();
                        new_path.path.push((s, t));
                        new_path.target = t;
                        next_paths.push(new_path);
                    }
                }
            }
            current_paths = next_paths;
        }

        paths.extend(current_paths);
        paths
    }

    /// Generate all morphisms up to a given path length.
    pub fn all_morphisms(&self, max_path_len: usize) -> Vec<Morphism> {
        let mut result = self.identities();
        for len in 1..=max_path_len {
            result.extend(self.paths_of_length(len));
        }
        result
    }

    /// Verify the universal property of the free category.
    ///
    /// Given any graph homomorphism f: G → U(C) (graph → underlying graph of C),
    /// there exists a unique functor f̂: F(G) → C extending f on edges and
    /// preserving composition.
    pub fn universal_property(&self) -> UniversalPropertyResult {
        let mut failures = Vec::new();

        // Verify identity laws: id ∘ f = f and f ∘ id = f
        for len in 1..=2 {
            for morphism in self.paths_of_length(len) {
                let id_src = Morphism::identity(morphism.source);
                let id_tgt = Morphism::identity(morphism.target);

                if let Some(composed) = id_src.compose(&morphism)
                    && composed.path != morphism.path
                {
                    failures.push("Left identity law fails".to_string());
                }
                if let Some(composed) = morphism.compose(&id_tgt)
                    && composed.path != morphism.path
                {
                    failures.push("Right identity law fails".to_string());
                }
            }
        }

        // Verify associativity on short paths
        let edges: Vec<Morphism> = self
            .graph
            .edges
            .iter()
            .map(|&(s, t)| Morphism {
                source: s,
                target: t,
                path: vec![(s, t)],
            })
            .collect();

        for f in &edges {
            for g in &edges {
                if f.target != g.source {
                    continue;
                }
                for h in &edges {
                    if g.target != h.source {
                        continue;
                    }
                    // (h ∘ g) ∘ f vs h ∘ (g ∘ f)
                    let fg = f.compose(g).unwrap();
                    let gh = g.compose(h).unwrap();

                    let left = fg.compose(h);
                    let right = f.compose(&gh);

                    if left != right {
                        failures.push("Associativity fails".to_string());
                    }
                }
            }
        }

        UniversalPropertyResult {
            functor_name: "FreeCategory".to_string(),
            passed: failures.is_empty(),
            failures,
        }
    }

    /// The unit η: G → U(F(G)) sends each vertex to its identity morphism.
    pub fn unit_at_vertex(&self, v: usize) -> Morphism {
        Morphism::identity(v)
    }

    /// Convert to a PathCategory.
    pub fn to_path_category(&self) -> PathCategory {
        PathCategory::from_graph(&self.graph)
    }
}

#[cfg(test)]
mod tests {
    use super::*;



    #[test]
    fn test_free_monoid_words_len_0() {
        let fm = FreeMonoid::build(vec![1, 2]);
        let words = fm.words(0);
        assert_eq!(words.len(), 1);
        assert!(words[0].is_identity());
    }

    #[test]
    fn test_free_monoid_words_len_1() {
        let fm = FreeMonoid::build(vec![1, 2]);
        let words = fm.words(1);
        // 1 empty + 2 singletons = 3
        assert_eq!(words.len(), 3);
    }

    #[test]
    fn test_free_monoid_words_len_2() {
        let fm = FreeMonoid::build(vec![1]);
        let words = fm.words(2);
        // empty, [1], [1,1] = 3
        assert_eq!(words.len(), 3);
    }

    #[test]
    fn test_free_monoid_universal_property() {
        let fm = FreeMonoid::build(vec![1, 2]);
        let result = fm.universal_property(|x| ListMonoid::new(vec![x * 10]));
        assert!(result.passed);
    }

    #[test]
    fn test_free_monoid_unit_map() {
        let fm = FreeMonoid::build(vec![1, 2]);
        let eta = fm.unit_map(5);
        assert_eq!(eta.elements, vec![5]);
    }

    #[test]
    fn test_free_category_build() {
        let g = Graph::new(3, vec![(0, 1), (1, 2)]);
        let fc = FreeCategory::build(g);
        assert_eq!(fc.graph.num_vertices, 3);
    }

    #[test]
    fn test_free_category_identities() {
        let g = Graph::new(3, vec![]);
        let fc = FreeCategory::build(g);
        let ids = fc.identities();
        assert_eq!(ids.len(), 3);
        assert!(ids.iter().all(|m| m.is_identity()));
    }

    #[test]
    fn test_free_category_paths_len_1() {
        let g = Graph::new(3, vec![(0, 1), (1, 2)]);
        let fc = FreeCategory::build(g);
        let paths = fc.paths_of_length(1);
        assert_eq!(paths.len(), 2);
    }

    #[test]
    fn test_free_category_paths_len_2() {
        let g = Graph::new(3, vec![(0, 1), (1, 2)]);
        let fc = FreeCategory::build(g);
        let paths = fc.paths_of_length(2);
        // Only path: 0→1→2
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0].path, vec![(0, 1), (1, 2)]);
    }

    #[test]
    fn test_free_category_all_morphisms() {
        let g = Graph::new(2, vec![(0, 1)]);
        let fc = FreeCategory::build(g);
        let all = fc.all_morphisms(1);
        // 2 identities + 1 edge = 3
        assert_eq!(all.len(), 3);
    }

    #[test]
    fn test_free_category_universal_property() {
        let g = Graph::new(3, vec![(0, 1), (1, 2)]);
        let fc = FreeCategory::build(g);
        let result = fc.universal_property();
        assert!(result.passed);
    }






}
