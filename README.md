# adjunction

**Category theory adjunctions for agent composition** — concrete implementations of free, forgetful, and reflective functors using actual mathematical objects.

[![crate](https://img.shields.io/crates/v/adjunction.svg)](https://crates.io/crates/adjunction)
[![docs](https://docs.rs/adjunction/badge.svg)](https://docs.rs/adjunction)

---

## Table of Contents

- [Overview](#overview)
- [Theory](#theory)
  - [What is an Adjunction?](#what-is-an-adjunction)
  - [The Triangle Identities](#the-triangle-identities)
  - [Free ⊣ Forgetful](#free--forgetful)
  - [Reflective Subcategories](#reflective-subcategories)
- [Modules](#modules)
- [Design Decisions](#design-decisions)
- [Examples](#examples)
  - [Example 1: Free Monoid Adjunction](#example-1-free-monoid-adjunction)
  - [Example 2: Path Category from a Graph](#example-2-path-category-from-a-graph)
  - [Example 3: Abelianization of a Group](#example-3-abelianization-of-a-group)
- [ASCII Reference](#ascii-reference)
- [API Reference](#api-reference)
- [References](#references)
- [License](#license)

---

## Overview

`adjunction` is a Rust library that provides **concrete, computational** implementations of fundamental category theory constructs. Rather than abstract trait hierarchies that are impossible to debug, every type here operates on real mathematical objects: lists, graphs, group presentations as `Vec<String>`.

This crate is designed for:

- **Agent composition frameworks** where functors represent transformations between agent domains
- **Mathematics education** — see the theory actually compute
- **Formal verification** — triangle identities are checked, not assumed
- **Anyone who wants category theory they can run, not just read about**

### What's Inside

| Module | What It Does | Key Types |
|--------|-------------|-----------|
| `adjunction` | Core struct with triangle identity verification | `Adjunction<A, B>`, `VerificationResult` |
| `unit` | Unit natural transformations η: 1 → R∘L | `Unit<A, B>`, `Morphism` |
| `counit` | Counit natural transformations ε: L∘R → 1 | `Counit<B>`, `ListMonoid`, `Graph` |
| `free` | Free functor implementations | `FreeMonoid`, `FreeCategory` |
| `forgetful` | Forgetful functor implementations | `ForgetMonoid`, `ForgetCategory` |
| `reflective` | Reflective subcategory: abelianization | `GroupPresentation`, `Abelianization` |

---

## Theory

### What is an Adjunction?

An **adjunction** is one of the most important structures in category theory. Given categories **C** and **D**, an adjunction F ⊣ G consists of:

1. A **left adjoint** functor F: C → D
2. A **right adjoint** functor G: D → C
3. A **unit** natural transformation η: 1_C → G∘F
4. A **counit** natural transformation ε: F∘G → 1_D

The fundamental idea: F and G are "approximately inverse" to each other, and η and ε measure exactly how close they come to being true inverses.

Formally, an adjunction can be defined in three equivalent ways (Mac Lane, Ch. IV):

1. **Via natural transformations** (unit-counit definition): η and ε satisfy the triangle identities
2. **Via hom-set isomorphisms**: Hom_D(FX, Y) ≅ Hom_C(X, GY) naturally in X and Y
3. **Via universal morphisms**: For each X in C, there is a universal arrow from X to G

This crate uses definition (1) because it's the most computational: we can actually *evaluate* η and ε on concrete objects and check whether the triangle identities hold.

### The Triangle Identities

The triangle identities are the *defining equations* of an adjunction. For all objects X in C and Y in D:

```
ε_{F(X)} ∘ F(η_X) = id_{F(X)}     (left triangle)
G(ε_Y) ∘ η_{G(Y)} = id_{G(Y)}     (right triangle)
```

In this crate, `Adjunction::verify_triangle_identities()` checks both identities computationally over a set of sample objects. If you provide enough samples, you get high confidence that the adjunction is valid.

Visually:

```
        Triangle Identity 1                    Triangle Identity 2

    F(X) ──── ε_{F(X)} ────▶ F(X)       G(Y) ──── η_{G(Y)} ────▶ G(F(G(Y)))
     │                               │                              │
  F(η_X)                          G(ε_Y)                      G(ε_Y)
     │                               │                              │
     ▼                               ▼                              ▼
 F(G(F(X)))                        G(Y) ◀──── id ────── G(F(G(Y)))


    Both diagrams commute: going around either way gives the same result.
```

### Free ⊣ Forgetful

The most important class of adjunctions in mathematics is the **free-forgetful** adjunction. The pattern is universal:

- The **free functor** F builds algebraic structure from bare data (sets → monoids, graphs → categories)
- The **forgetful functor** U strips structure away (monoids → sets, categories → graphs)
- F is left adjoint to U: F ⊣ U

The **universal property** of the free functor says: given any function from the generators to a structured object, there is a *unique* structure-preserving map extending it. This is what makes free objects "free" — they have no extra relations beyond what the structure requires.

**Examples in this crate:**

| Free Functor | Forgetful Functor | What F Does | What U Does |
|-------------|-------------------|-------------|-------------|
| `FreeMonoid` | `ForgetMonoid` | Set → lists under concatenation | Strip monoid wrapper |
| `FreeCategory` | `ForgetCategory` | Graph → paths under composition | Keep only edges |

### Reflective Subcategories

A **reflective subcategory** D of C is a full subcategory where the inclusion functor i: D → C has a **left adjoint** L: C → D. The left adjoint L is called the **reflector** — it takes any object of C and "reflects" it into D in the most economical way.

The prototypical example: **Abelianization**.

- **C** = Groups (all groups)
- **D** = Abelian groups
- **i** = inclusion of abelian groups into all groups
- **L** = G ↦ G/[G,G] (force commutativity)

The unit η: G → i(L(G)) is the quotient map. The counit ε: L(i(A)) → A is an isomorphism (already-abelian groups don't change).

This crate implements abelianization concretely: given a group presentation (generators and relations), it adds commutativity relations ab = ba for all pairs of generators and verifies the resulting structure is a valid reflective subcategory.

---

## Design Decisions

### Concrete over Abstract

Every type in this crate operates on real data:

- **Lists** (`Vec<i32>`, `Vec<String>`) for monoid elements
- **Graphs** (`Graph` with vertex count + edge list) for categories
- **Group presentations** (generators as `Vec<String>`, relations as word pairs)

There are no `impl Functor` traits, no HKT bounds, no associated types. You call methods on structs and get answers back. This is intentional: abstract category theory frameworks exist (and are beautiful), but they're hard to learn from and harder to debug.

### Why Function Fields Are `Box<dyn Fn>`

The `Adjunction` struct stores its unit, counit, and functors as `Box<dyn Fn>` rather than generic parameters. This means:

- You can construct adjunctions from closures, not just function pointers
- The type signature stays readable: `Adjunction<A, B>` instead of `Adjunction<A, B, F, G, Unit, Counit, EqA, EqB>`
- Serialization skips these fields (you can't serialize functions), but the *names* serialize

### Why `serde`?

Agent composition frameworks need to serialize and transmit configurations. All data-carrying types derive `Serialize` and `Deserialize`. The function-carrying types (`Adjunction`, `Unit`, `Counit`) serialize their metadata but skip the closures — you reconstruct them programmatically.

### Edition 2024

This crate uses Rust Edition 2024 for the latest language features and borrow checker improvements.

---

## Examples

### Example 1: Free Monoid Adjunction

The free monoid on a set S is the set of all finite lists of elements of S, with concatenation as the monoid operation.

```rust
use adjunction::free::FreeMonoid;
use adjunction::counit::ListMonoid;

// Build the free monoid on {1, 2}
let fm = FreeMonoid::build(vec![1, 2]);

// Generate all words up to length 2
let words = fm.words(2);
// words = [], [1], [2], [1,1], [1,2], [2,1], [2,2]
assert_eq!(words.len(), 7);

// The unit η sends 5 to [5]
let singleton = fm.unit_map(5);
assert_eq!(singleton.elements, vec![5]);

// Verify the universal property with a map f(x) = [x * 10]
let result = fm.universal_property(|x| ListMonoid::new(vec![x * 10]));
assert!(result.passed);
```

### Example 2: Path Category from a Graph

The free category on a graph has vertices as objects and paths as morphisms.

```rust
use adjunction::free::FreeCategory;
use adjunction::counit::Graph;

// Build a graph: 0 → 1 → 2
let graph = Graph::new(3, vec![(0, 1), (1, 2)]);
let fc = FreeCategory::build(graph);

// Get identity morphisms (one per vertex)
let ids = fc.identities();
assert_eq!(ids.len(), 3);
assert!(ids.iter().all(|m| m.is_identity()));

// Get paths of length 2: only 0→1→2
let paths = fc.paths_of_length(2);
assert_eq!(paths.len(), 1);
assert_eq!(paths[0].path, vec![(0, 1), (1, 2)]);

// Verify the universal property (identity laws + associativity)
let result = fc.universal_property();
assert!(result.passed);
```

### Example 3: Abelianization of a Group

Abelianization forces commutativity, producing the "closest" abelian group.

```rust
use adjunction::reflective::{GroupPresentation, Abelianization};

// The free group on {a, b} — non-abelian
let g = GroupPresentation::free(vec!["a".into(), "b".into()]);
assert!(!g.is_abelian());

// Abelianize: add ab = ba
let ab = Abelianization::abelianize(&g);
assert!(ab.verify_abelian());
assert_eq!(ab.commutator_relations.len(), 1); // ab = ba

// Verify this forms a reflective subcategory
let result = ab.verify_reflective();
assert!(result.is_reflective);

// The dihedral group D₃ is non-abelian
let d3 = GroupPresentation::dihedral(3);
assert!(!d3.is_abelian());

// But its abelianization is abelian (and reflective)
let ab_d3 = Abelianization::abelianize(&d3);
assert!(ab_d3.verify_abelian());
assert!(ab_d3.verify_reflective().is_reflective);
```

---

## ASCII Reference

### Adjunction Triangle Diagram

```
                    CATEGORICAL                            COMPUTATIONAL
                    ───────────                            ─────────────

       C ───── F ────▶ D                    Set ─── FreeMonoid ───▶ Monoid
       │               │                    │                        │
       │               │                    │  η(x) = [x]            │
     η ↓            ε  ↓                  ↓                        ↓
       │               │               {[x]}                    ListMonoid
       │               │                    │                        │
       C ◀──── G ──── D                    Set ◀── ForgetMonoid ◀── Monoid
                                                ε unwraps

        Triangle Identity 1:                Triangle Identity 2:
        ε_{F(X)} ∘ F(η_X) = id_{F(X)}      G(ε_Y) ∘ η_{G(Y)} = id_{G(Y)}

        For lists:                          For lists:
        ε([x]) = [x]                       η(x) = [x]
        F(η(x)) = [x]                      G(ε([x])) = x
        ε([x]) = [x] ✓                     η(x) = x ... ✓
```

### Free-Forgetful Adjunction (General Pattern)

```
    ┌─────────────────────────────────────────────────────────┐
    │                                                         │
    │   "Structured"                          "Unstructured"   │
    │   Category D                            Category C      │
    │                                                         │
    │   ┌──────────┐         F (Free)       ┌──────────┐     │
    │   │          │ ◀──────────────────── │          │     │
    │   │ Monoids  │                        │  Sets    │     │
    │   │ Categories│ ────────────────────▶ │ Graphs   │     │
    │   │  Groups  │         U (Forget)     │ Presets  │     │
    │   └──────────┘                        └──────────┘     │
    │                                                         │
    │   F ⊣ U means:                                          │
    │   Hom_D(F(X), Y) ≅ Hom_C(X, U(Y))                      │
    │                                                         │
    │   "Maps from free objects = maps from generators"        │
    │                                                         │
    └─────────────────────────────────────────────────────────┘
```

### Reflective Subcategory (Abelianization)

```
    Groups ──── L (Abelianize) ────▶ Abelian Groups
       │                                   │
       │  η: G → G/[G,G]                  │  ε: A ≅ A (iso)
       │  (quotient map)                   │  (already abelian)
       │                                   │
       ◀──── i (Inclusion) ────────────── ◀

    L is a REFLECTOR: the "best approximation" of a group
    by an abelian group. For any homomorphism f: G → A
    where A is abelian, f factors uniquely through L(G).
```

---

## API Reference

### `adjunction` Module

- **`Adjunction<A, B>`** — Core adjunction struct
  - `new(left_name, right_name, unit, counit, left_functor, right_functor, eq_a, eq_b)` — Constructor
  - `apply_left(&A) -> B` — Apply the left adjoint
  - `apply_right(&B) -> A` — Apply the right adjoint
  - `apply_unit(&A) -> A` — Apply the unit η
  - `apply_counit(&B) -> B` — Apply the counit ε
  - `verify_triangle_identities(src, tgt) -> bool` — Check both triangle identities
- **`VerificationResult`** — Serializable verification report

### `unit` Module

- **`Unit<A, B>`** — Unit natural transformation
  - `new(name, transform)` — Constructor
  - `apply(A) -> B` — Apply the unit
- **`Morphism`** — A morphism in the path category
  - `identity(v)` — Create identity morphism
  - `compose(&other)` — Compose two morphisms
  - `is_identity()` — Check if identity
- `free_monoid_unit()` — Unit for FreeMonoid: x ↦ [x]
- `free_monoid_unit_string()` — String version
- `free_category_unit()` — Unit for FreeCategory: v ↦ id_v

### `counit` Module

- **`Counit<B>`** — Counit natural transformation
  - `new(name, transform)` — Constructor
  - `apply(&B) -> B` — Apply the counit
- **`ListMonoid`** — Monoid as list under concatenation
  - `new(elements)`, `multiply(&other)`, `identity()`, `is_identity()`
- **`Graph`** — Directed graph (vertex count + edge list)
  - `new(num_vertices, edges)`, `add_edge(from, to)`, `vertices()`, `edges_from(v)`
- **`PathCategory`** — Category from a graph
  - `from_graph(&Graph)`, `identities()`
- `forgetful_monoid_counit()` — Counit for ForgetMonoid
- `forgetful_category_counit()` — Counit for ForgetCategory

### `free` Module

- **`FreeMonoid`** — Free monoid functor
  - `build(generators)` — Construct
  - `words(max_len)` — Generate all words up to length
  - `universal_property(target_map)` — Verify universal property
  - `unit_map(x)` — Unit η: x ↦ [x]
- **`FreeCategory`** — Free category functor
  - `build(graph)` — Construct
  - `identities()` — Identity morphisms
  - `paths_of_length(len)` — Paths of exact length
  - `all_morphisms(max_len)` — All morphisms up to length
  - `universal_property()` — Verify (identity laws + associativity)
- **`UniversalPropertyResult`** — Verification result

### `forgetful` Module

- **`ForgetMonoid`** — Forget monoid structure
  - `apply(&ListMonoid) -> Vec<i32>` — Extract elements
  - `count(&ListMonoid) -> usize` — Element count
  - `is_empty(&ListMonoid) -> bool` — Check identity
- **`ForgetCategory`** — Forget category structure
  - `apply(&PathCategory) -> Graph` — Extract graph
  - `edges(&PathCategory) -> Vec<Edge>` — Extract edges
  - `num_objects(&PathCategory) -> usize` — Count objects
  - `is_empty(&PathCategory) -> bool` — Check emptiness
- **`MonoidPresentation`** — Monoid with explicit multiplication table
- **`ForgetGroup`** — Forget group structure

### `reflective` Module

- **`GroupPresentation`** — Group by generators and relations
  - `new(generators, relations)` — Constructor
  - `free(generators)` — Free group (no relations)
  - `cyclic(n)` — Z/nZ
  - `dihedral(n)` — D_n
  - `is_abelian()` — Check commutativity
- **`Abelianization`** — Result of abelianizing a group
  - `abelianize(&GroupPresentation)` — Force commutativity
  - `verify_abelian()` — Check result is abelian
  - `verify_reflective()` — Check reflective subcategory conditions
- **`ReflectiveVerification`** — Verification result

---

## References

1. **Mac Lane, Saunders.** *Categories for the Working Mathematician.* 2nd ed., Springer, 1971. — The foundational text. Chapter IV defines adjunctions via unit-counit, hom-set, and universal morphism formulations. Our `verify_triangle_identities()` directly implements the triangle identities from Theorem IV.2.

2. **Awodey, Steve.** *Category Theory.* 2nd ed., Oxford University Press, 2010. — A gentler introduction. Chapter 9 covers adjunctions with excellent motivation. The free-forgetful pattern (our `free` and `forgetful` modules) follows Awodey's treatment closely.

3. **Riehl, Emily.** *Category Theory in Context.* Dover, 2016. — Modern presentation with the slogan "adjunctions arise everywhere." Chapter 4 provides the clearest treatment of free-forgetful adjunctions we've seen. Available free at [math.jhu.edu/~eriehl/context.pdf](http://math.jhu.edu/~eriehl/context.pdf).

4. **Leinster, Tom.** *Basic Category Theory.* Cambridge University Press, 2014. — Compact and precise. Chapters 2-3 cover functors and adjunctions. The definition of reflective subcategories (our `reflective` module) follows Leinster's treatment. Available free at [arxiv.org/abs/1612.09375](https://arxiv.org/abs/1612.09375).

5. **Borceux, Francis.** *Handbook of Categorical Algebra, Volume 2.* Cambridge University Press, 1994. — Encyclopedic reference. Chapter 4 covers adjunctions in full generality including limits and colimits.

6. **nLab authors.** "Adjunction." *nLab*, 2024. — Online encyclopedia entry with connections to logic, topology, and computer science. Available at [ncatlab.org/nlab/show/adjunction](https://ncatlab.org/nlab/show/adjunction).

7. **Pierce, Benjamin C.** *Basic Category Theory for Computer Scientists.* MIT Press, 1991. — Short and practical. Shows how adjunctions appear in type theory and programming language semantics, motivating the computational approach this crate takes.

---

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
