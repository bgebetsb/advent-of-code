use std::ops::{Range, RangeInclusive};

use num_traits::Num;

pub trait RangeHandling<T> {
    fn deduplicate_ranges(&self) -> Vec<Range<T>>;
}

impl<T> RangeHandling<T> for Vec<Range<T>>
where
    T: Copy + Ord + Num,
{
    fn deduplicate_ranges(&self) -> Vec<Range<T>> {
        let mut sorted = self.clone();
        sorted.sort_unstable_by_key(|range| range.start);
        let mut new_ranges: Vec<Range<T>> = Vec::new();

        for range in sorted {
            match new_ranges.iter().next_back() {
                Some(last) => {
                    if range.end > last.end {
                        let start = last.end.max(range.start);
                        new_ranges.push(start..range.end);
                    }
                }
                None => new_ranges.push(range),
            }
        }

        new_ranges
    }
}

impl<T> RangeHandling<T> for Vec<RangeInclusive<T>>
where
    T: Copy + Ord + Num,
{
    fn deduplicate_ranges(&self) -> Vec<Range<T>> {
        let ranges: Vec<_> = self
            .iter()
            .map(|range| *range.start()..*range.end() + T::one())
            .collect();

        ranges.deduplicate_ranges()
    }
}
