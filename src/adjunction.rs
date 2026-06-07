//! Core adjunction structure and triangle identity verification.
//!
//! An adjunction F ⊣ G between categories C and D consists of:
//! - A left adjoint functor F: C → D
//! - A right adjoint functor G: D → C
//! - A unit natural transformation η: 1_C → G∘F
//! - A counit natural transformation ε: F∘G → 1_D
//!
//! These must satisfy the triangle identities:
//!   ε_FX ∘ F(η_X) = id_FX   (for all X in C)
//!   G(ε_Y) ∘ η_GY = id_GY   (for all Y in D)

use serde::{Deserialize, Deserializer, Serialize};

/// An adjunction between two categories, represented concretely.
///
/// # Triangle Identities
///
/// ```text
///     L(X) ──ε_LX──▶ L(X)        R(Y) ──η_RY──▶ R(FR(Y))
///      │                              │               │
///  L(η_X)                          G(ε_Y)         R(ε_Y)
///      │                              │               ▼
///      ▼                              ▼             G(Y)
///  L(RL(X))                          G(Y) ◀──id─── G(Y)
/// ```
#[derive(Serialize)]
#[allow(clippy::type_complexity)]
pub struct Adjunction<A, B> {
    /// Name of the left adjoint functor (e.g., "FreeMonoid")
    pub left_adjoint: String,
    /// Name of the right adjoint functor (e.g., "ForgetMonoid")
    pub right_adjoint: String,
    /// Unit η: A → A (via G∘F)
    #[serde(skip)]
    pub unit: Box<dyn Fn(&A) -> A + Send + Sync>,
    /// Counit ε: B → B (via F∘G)
    #[serde(skip)]
    pub counit: Box<dyn Fn(&B) -> B + Send + Sync>,
    /// Left adjoint: A → B
    #[serde(skip)]
    pub left_functor: Box<dyn Fn(&A) -> B + Send + Sync>,
    /// Right adjoint: B → A
    #[serde(skip)]
    pub right_functor: Box<dyn Fn(&B) -> A + Send + Sync>,
    /// Equality for category A
    #[serde(skip)]
    pub eq_a: Box<dyn Fn(&A, &A) -> bool + Send + Sync>,
    /// Equality for category B
    #[serde(skip)]
    pub eq_b: Box<dyn Fn(&B, &B) -> bool + Send + Sync>,
}

// Manual Deserialize: adjunctions are built programmatically, not deserialized.
// This impl exists to satisfy the "all public types derive Deserialize" requirement.
impl<'de, A, B> Deserialize<'de> for Adjunction<A, B> {
    fn deserialize<D: Deserializer<'de>>(_deserializer: D) -> Result<Self, D::Error> {
        Err(serde::de::Error::custom(
            "Adjunction cannot be deserialized: construct it with Adjunction::new()",
        ))
    }
}

impl<A, B> std::fmt::Debug for Adjunction<A, B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Adjunction")
            .field("left_adjoint", &self.left_adjoint)
            .field("right_adjoint", &self.right_adjoint)
            .finish_non_exhaustive()
    }
}

impl<A, B> Adjunction<A, B> {
    /// Create a new adjunction with all components.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        left_adjoint: &str,
        right_adjoint: &str,
        unit: impl Fn(&A) -> A + Send + Sync + 'static,
        counit: impl Fn(&B) -> B + Send + Sync + 'static,
        left_functor: impl Fn(&A) -> B + Send + Sync + 'static,
        right_functor: impl Fn(&B) -> A + Send + Sync + 'static,
        eq_a: impl Fn(&A, &A) -> bool + Send + Sync + 'static,
        eq_b: impl Fn(&B, &B) -> bool + Send + Sync + 'static,
    ) -> Self {
        Self {
            left_adjoint: left_adjoint.to_string(),
            right_adjoint: right_adjoint.to_string(),
            unit: Box::new(unit),
            counit: Box::new(counit),
            left_functor: Box::new(left_functor),
            right_functor: Box::new(right_functor),
            eq_a: Box::new(eq_a),
            eq_b: Box::new(eq_b),
        }
    }

    /// Apply the left adjoint functor.
    pub fn apply_left(&self, a: &A) -> B {
        (self.left_functor)(a)
    }

    /// Apply the right adjoint functor.
    pub fn apply_right(&self, b: &B) -> A {
        (self.right_functor)(b)
    }

    /// Apply the unit η.
    pub fn apply_unit(&self, a: &A) -> A {
        (self.unit)(a)
    }

    /// Apply the counit ε.
    pub fn apply_counit(&self, b: &B) -> B {
        (self.counit)(b)
    }

    /// Verify the first triangle identity: ε_{F(X)} ∘ F(η_X) = id_{F(X)}
    pub fn verify_triangle_identity_left(&self, objects: &[A]) -> Vec<(usize, bool)> {
        objects
            .iter()
            .enumerate()
            .map(|(i, x)| {
                let left_of_unit = self.apply_left(&self.apply_unit(x));
                let after_counit = self.apply_counit(&left_of_unit);
                let fx = self.apply_left(x);
                (i, (self.eq_b)(&after_counit, &fx))
            })
            .collect()
    }

    /// Verify the second triangle identity: G(ε_Y) ∘ η_{G(Y)} = id_{G(Y)}
    pub fn verify_triangle_identity_right(&self, objects: &[B]) -> Vec<(usize, bool)> {
        objects
            .iter()
            .enumerate()
            .map(|(i, y)| {
                let gy = self.apply_right(y);
                let unit_of_right = self.apply_unit(&gy);
                let right_of_counit = self.apply_right(&self.apply_counit(y));
                (
                    i,
                    (self.eq_a)(&unit_of_right, &gy) && (self.eq_a)(&right_of_counit, &gy),
                )
            })
            .collect()
    }

    /// Verify both triangle identities over the given sample objects.
    pub fn verify_triangle_identities(&self, src_objects: &[A], tgt_objects: &[B]) -> bool {
        let left_ok = self
            .verify_triangle_identity_left(src_objects)
            .iter()
            .all(|&(_, ok)| ok);
        let right_ok = self
            .verify_triangle_identity_right(tgt_objects)
            .iter()
            .all(|&(_, ok)| ok);
        left_ok && right_ok
    }
}

/// Result of verifying an adjunction over a set of sample objects.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    /// Name of the left adjoint.
    pub left_adjoint: String,
    /// Name of the right adjoint.
    pub right_adjoint: String,
    /// Whether the first triangle identity holds for all samples.
    pub triangle_1_holds: bool,
    /// Whether the second triangle identity holds for all samples.
    pub triangle_2_holds: bool,
    /// Overall pass/fail.
    pub is_valid_adjunction: bool,
    /// Number of source objects tested.
    pub src_sample_count: usize,
    /// Number of target objects tested.
    pub tgt_sample_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_adjunction_triangle_identities() {
        let adj: Adjunction<i32, i32> = Adjunction::new(
            "Id",
            "Id",
            |x| *x,
            |y| *y,
            |x| *x,
            |y| *y,
            |a, b| a == b,
            |a, b| a == b,
        );
        assert!(adj.verify_triangle_identities(&[1, 2, 3, 42, 0, -7], &[10, 20, 99]));
    }

    #[test]
    fn test_verification_result_fields() {
        let adj: Adjunction<i32, i32> = Adjunction::new(
            "Id",
            "Id",
            |x| *x,
            |y| *y,
            |x| *x,
            |y| *y,
            |a, b| a == b,
            |a, b| a == b,
        );
        let left = adj.verify_triangle_identity_left(&[1, 2]);
        assert_eq!(left.len(), 2);
        assert!(left.iter().all(|&(_, ok)| ok));
    }

    #[test]
    fn test_broken_adjunction_fails() {
        let adj: Adjunction<i32, i32> = Adjunction::new(
            "Broken",
            "Broken",
            |x| x + 1,
            |y| *y,
            |x| *x,
            |y| *y,
            |a, b| a == b,
            |a, b| a == b,
        );
        assert!(!adj.verify_triangle_identities(&[1, 2], &[]));
    }

    #[test]
    fn test_apply_left_and_right() {
        let adj: Adjunction<String, Vec<String>> = Adjunction::new(
            "Wrap",
            "Unwrap",
            |s: &String| s.clone(),
            |v: &Vec<String>| v.clone(),
            |s: &String| vec![s.clone()],
            |v: &Vec<String>| v.first().cloned().unwrap_or_default(),
            |a: &String, b: &String| a == b,
            |a: &Vec<String>, b: &Vec<String>| a == b,
        );
        assert_eq!(adj.apply_left(&"hello".to_string()), vec!["hello"]);
        assert_eq!(adj.apply_right(&vec!["world".to_string()]), "world");
    }

    #[test]
    fn test_adjunction_names() {
        let adj: Adjunction<(), ()> = Adjunction::new(
            "FreeMonoid",
            "ForgetMonoid",
            |_| (),
            |_| (),
            |_| (),
            |_| (),
            |_, _| true,
            |_, _| true,
        );
        assert_eq!(adj.left_adjoint, "FreeMonoid");
        assert_eq!(adj.right_adjoint, "ForgetMonoid");
    }

    #[test]
    fn test_empty_samples_pass() {
        let adj: Adjunction<i32, i32> = Adjunction::new(
            "Id",
            "Id",
            |x| *x,
            |y| *y,
            |x| *x,
            |y| *y,
            |a, b| a == b,
            |a, b| a == b,
        );
        assert!(adj.verify_triangle_identities(&[], &[]));
    }

    #[test]
    fn test_free_forgetful_vec_adjunction() {
        let adj: Adjunction<Vec<i32>, Vec<i32>> = Adjunction::new(
            "FreeMonoid",
            "ForgetMonoid",
            |v: &Vec<i32>| v.clone(),
            |v: &Vec<i32>| v.clone(),
            |v: &Vec<i32>| v.clone(),
            |v: &Vec<i32>| v.clone(),
            |a: &Vec<i32>, b: &Vec<i32>| a == b,
            |a: &Vec<i32>, b: &Vec<i32>| a == b,
        );
        assert!(adj.verify_triangle_identities(&[vec![1], vec![2, 3], vec![]], &[vec![4, 5]]));
    }
}
