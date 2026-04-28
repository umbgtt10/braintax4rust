//! # braintax
//!
//! A minimal utility crate for estimating the cognitive "tax" of code patterns.
//!
//! This crate provides a simple scoring function that quantifies how mentally
//! taxing a given code structure is, based on nesting depth and branching
//! complexity. It is designed as a lightweight companion to complexity analysis
//! tools like `crap4rust`.
//!
//! ## Example
//!
//! ```rust
//! use braintax::Braintax;
//!
//! let score = Braintax::new()
//!     .with_nesting(3)
//!     .with_branches(5)
//!     .compute();
//!
//! assert!(score > 0);
//! ```

/// A cognitive "tax" estimator that combines nesting depth and branching
/// complexity into a single score.
///
/// The formula is intentionally simple:
/// `tax = nesting² + branches + 1`
///
/// This provides a rough heuristic: deeply nested code with many branches
/// is more mentally taxing to read and maintain.
#[derive(Clone, Copy, Debug, Default)]
pub struct Braintax {
    nesting: u32,
    branches: u32,
}

impl Braintax {
    /// Creates a new `Braintax` with zero nesting and zero branches.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the nesting depth (e.g., how many levels deep control flow is nested).
    #[must_use]
    pub const fn with_nesting(mut self, nesting: u32) -> Self {
        self.nesting = nesting;
        self
    }

    /// Sets the number of branching points (e.g., `if`, `match`, `loop` constructs).
    #[must_use]
    pub const fn with_branches(mut self, branches: u32) -> Self {
        self.branches = branches;
        self
    }

    /// Computes the cognitive tax score.
    ///
    /// # Formula
    ///
    /// ```text
    /// tax = nesting² + branches + 1
    /// ```
    ///
    /// A score of **1** represents the baseline (no nesting, no branches).
    /// Higher scores indicate increasing mental taxation.
    #[must_use]
    pub const fn compute(&self) -> u64 {
        (self.nesting as u64).pow(2) + self.branches as u64 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn baseline_returns_one() {
        let tax = Braintax::new().compute();
        assert_eq!(tax, 1);
    }

    #[test]
    fn nesting_increases_score_quadratically() {
        let tax = Braintax::new().with_nesting(3).compute();
        assert_eq!(tax, 10); // 3² + 0 + 1 = 10
    }

    #[test]
    fn branches_add_linearly() {
        let tax = Braintax::new().with_branches(5).compute();
        assert_eq!(tax, 6); // 0² + 5 + 1 = 6
    }

    #[test]
    fn combined_nesting_and_branches() {
        let tax = Braintax::new().with_nesting(2).with_branches(4).compute();
        assert_eq!(tax, 9); // 2² + 4 + 1 = 9
    }

    #[test]
    fn large_values_do_not_overflow() {
        let tax = Braintax::new()
            .with_nesting(1_000)
            .with_branches(100_000)
            .compute();
        assert_eq!(tax, 1_100_001); // 1000² + 100000 + 1 = 1_100_001
    }

    #[test]
    fn debug_format_does_not_panic() {
        let tax = Braintax::new().with_nesting(1).with_branches(2);
        let _ = format!("{tax:?}");
    }

    #[test]
    fn clone_produces_equal_score() {
        let original = Braintax::new().with_nesting(4).with_branches(3);
        let cloned = original;
        assert_eq!(original.compute(), cloned.compute());
    }
}
