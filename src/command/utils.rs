use std::ops;

use nu_protocol::{LabeledError, Range};

pub fn to_u32_range(r: Box<Range>) -> Result<ops::Range<u32>, LabeledError> {
    let (start, end) = match r.as_ref() {
        Range::IntRange(r) => (
            r.start() as u32,
            match r.end() {
                ops::Bound::Included(end) => (end + 1) as u32,
                ops::Bound::Excluded(end) => end as u32,
                ops::Bound::Unbounded => u32::MAX,
            },
        ),
        _ => return Err(LabeledError::new("Expected an integer range")),
    };

    Ok(start..end)
}
