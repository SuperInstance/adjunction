//! # adjunction
//!
//! Category theory adjunctions for agent composition — concrete implementations
//! of free, forgetful, and reflective functors using actual mathematical objects.
//!
//! ## Modules
//!
//! - [`adjunction`] — Core `Adjunction` struct with triangle identity verification
//! - [`unit`] — Unit natural transformations (η): from identity to R∘L
//! - [`counit`] — Counit natural transformations (ε): from L∘R to identity
//! - [`free`] — Free functor implementations: FreeMonoid, FreeCategory
//! - [`forgetful`] — Forgetful functor implementations: strip algebraic structure
//! - [`reflective`] — Reflective subcategories: abelianization of groups

pub mod adjunction;
pub mod counit;
pub mod forgetful;
pub mod free;
pub mod reflective;
pub mod unit;
