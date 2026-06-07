//! Unit natural transformations (η) for adjunctions.
//!
//! The unit η: 1_C → G∘F of an adjunction F ⊣ G maps each object in the
//! source category to its "free" image under the composition G∘F.
//!
//! For the free-forgetful adjunction on monoids:
//!   η(x) = [x]  (the singleton list containing x)
//!
//! For the free-forgetful adjunction on categories:
//!   η(v) = identity morphism on vertex v

use serde::{Deserialize, Deserializer, Serialize};

/// A unit natural transformation from type A to type B.
///
/// In the adjunction F ⊣ G, the unit η maps each object X to G(F(X)).
#[derive(Debug, Clone, Serialize)]
pub struct Unit<A, B> {
    /// Human-readable name (e.g., "FreeMonoid Unit")
    pub name: String,
    /// The underlying function η: A → B
    #[serde(skip)]
    pub transform: fn(A) -> B,
}

impl<'de, A, B> Deserialize<'de> for Unit<A, B> {
    fn deserialize<D: Deserializer<'de>>(_deserializer: D) -> Result<Self, D::Error> {
        Err(serde::de::Error::custom(
            "Unit cannot be deserialized: construct it with Unit::new()",
        ))
    }
}

impl<A, B> Unit<A, B> {
    /// Create a new named unit transformation.
    pub fn new(name: &str, transform: fn(A) -> B) -> Self {
        Self {
            name: name.to_string(),
            transform,
        }
    }

    /// Apply the unit transformation to an object.
    pub fn apply(&self, a: A) -> B {
        (self.transform)(a)
    }
}

/// A vertex in a graph, used for the FreeCategory unit.
pub type Vertex = usize;

/// A morphism in the path category: a path of edges from source to target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Morphism {
    /// Source vertex.
    pub source: Vertex,
    /// Target vertex.
    pub target: Vertex,
    /// Sequence of edges traversed (empty = identity morphism).
    pub path: Vec<(Vertex, Vertex)>,
}

impl Morphism {
    /// Create an identity morphism at a vertex (empty path).
    pub fn identity(v: Vertex) -> Self {
        Self {
            source: v,
            target: v,
            path: vec![],
        }
    }

    /// Compose two morphisms: g ∘ f, where f: A → B and g: B → C.
    pub fn compose(&self, other: &Morphism) -> Option<Morphism> {
        if self.target != other.source {
            return None;
        }
        let mut path = self.path.clone();
        path.extend_from_slice(&other.path);
        Some(Morphism {
            source: self.source,
            target: other.target,
            path,
        })
    }

    /// Check if this is an identity morphism.
    pub fn is_identity(&self) -> bool {
        self.source == self.target && self.path.is_empty()
    }
}

/// Unit for the FreeMonoid adjunction: η maps an element to a singleton list.
///
/// Given a set S, the free monoid on S consists of all finite lists of
/// elements of S with concatenation as the operation. The unit η: S → U(F(S))
/// sends each element x to the one-element list [x].
///
/// # Examples
/// ```
/// use adjunction::unit::free_monoid_unit;
///
/// let eta = free_monoid_unit();
/// assert_eq!(eta.apply(42), vec![42]);
/// ```
pub fn free_monoid_unit() -> Unit<i32, Vec<i32>> {
    Unit::new("FreeMonoid Unit η: S → U(F(S))", |x| vec![x])
}

/// Unit for the FreeMonoid adjunction (string version).
pub fn free_monoid_unit_string() -> Unit<String, Vec<String>> {
    Unit::new("FreeMonoid Unit η (String)", |s| vec![s])
}

/// Unit for the FreeCategory adjunction: η maps a vertex to its identity morphism.
///
/// The free category on a graph has:
/// - Objects = vertices of the graph
/// - Morphisms = paths in the graph (including empty paths = identities)
///
/// The unit η sends each vertex v to id_v, the identity morphism on v.
pub fn free_category_unit() -> Unit<Vertex, Morphism> {
    Unit::new("FreeCategory Unit η: V → id_v", |v| {
        Morphism::identity(v)
    })
}

/// Compute the unit for a specific vertex in the free category.
/// Returns the identity morphism at that vertex.
pub fn identity_morphism_at(v: Vertex) -> Morphism {
    Morphism::identity(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_free_monoid_unit_int() {
        let eta = free_monoid_unit();
        assert_eq!(eta.apply(5), vec![5]);
        assert_eq!(eta.apply(0), vec![0]);
        assert_eq!(eta.apply(-1), vec![-1]);
    }

    #[test]
    fn test_free_monoid_unit_string() {
        let eta = free_monoid_unit_string();
        assert_eq!(eta.apply("hello".to_string()), vec!["hello"]);
        assert_eq!(eta.apply("".to_string()), vec![""]);
    }

    #[test]
    fn test_free_category_unit() {
        let eta = free_category_unit();
        let m = eta.apply(0);
        assert!(m.is_identity());
        assert_eq!(m.source, 0);

        let m42 = eta.apply(42);
        assert!(m42.is_identity());
        assert_eq!(m42.source, 42);
    }

    #[test]
    fn test_identity_morphism() {
        let m = Morphism::identity(3);
        assert!(m.is_identity());
        assert_eq!(m.source, 3);
        assert_eq!(m.target, 3);
        assert!(m.path.is_empty());
    }

    #[test]
    fn test_morphism_compose() {
        let f = Morphism {
            source: 0,
            target: 1,
            path: vec![(0, 1)],
        };
        let g = Morphism {
            source: 1,
            target: 2,
            path: vec![(1, 2)],
        };
        let fg = f.compose(&g).unwrap();
        assert_eq!(fg.source, 0);
        assert_eq!(fg.target, 2);
        assert_eq!(fg.path, vec![(0, 1), (1, 2)]);
    }

    #[test]
    fn test_morphism_compose_unmatched() {
        let f = Morphism {
            source: 0,
            target: 1,
            path: vec![(0, 1)],
        };
        let g = Morphism {
            source: 2,
            target: 3,
            path: vec![(2, 3)],
        };
        assert!(f.compose(&g).is_none());
    }

    #[test]
    fn test_morphism_compose_with_identity() {
        let id = Morphism::identity(1);
        let f = Morphism {
            source: 0,
            target: 1,
            path: vec![(0, 1)],
        };
        let result = f.compose(&id).unwrap();
        assert_eq!(result.source, 0);
        assert_eq!(result.target, 1);
        assert_eq!(result.path, vec![(0, 1)]);
    }



    #[test]
    fn test_morphism_not_identity() {
        let m = Morphism {
            source: 0,
            target: 1,
            path: vec![(0, 1)],
        };
        assert!(!m.is_identity());
    }
}
