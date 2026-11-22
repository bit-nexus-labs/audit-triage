/// Minimum score for High priority (SPEC 3.3: 9 or more).
pub const PRIORITY_HIGH_MIN: u32 = 9;

/// Minimum score for Medium priority (SPEC 3.3: 6 to 8).
pub const PRIORITY_MEDIUM_MIN: u32 = 6;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Priority {
    High,
    Medium,
    Low,
}

pub fn priority_from_score(score: u32) -> Priority {
    if score >= PRIORITY_HIGH_MIN {
        Priority::High
    } else if score >= PRIORITY_MEDIUM_MIN {
        Priority::Medium
    } else {
        Priority::Low
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_boundaries() {
        assert_eq!(priority_from_score(5), Priority::Low);
        assert_eq!(priority_from_score(6), Priority::Medium);
        assert_eq!(priority_from_score(8), Priority::Medium);
        assert_eq!(priority_from_score(9), Priority::High);
    }
}
