use std::{
    collections::VecDeque,
    ops::{Range, RangeInclusive},
};

use num_traits::Num;

pub trait RangeHandling<T> {
    fn deduplicate_ranges(&self) -> Vec<RangeInclusive<T>>;
}

impl<T> RangeHandling<T> for Vec<Range<T>>
where
    T: Copy + PartialEq + PartialOrd + Num,
{
    fn deduplicate_ranges(&self) -> Vec<RangeInclusive<T>> {
        let ranges: Vec<_> = self
            .iter()
            .map(|range| range.start..=range.end - T::one())
            .collect();

        ranges.deduplicate_ranges()
    }
}

impl<T> RangeHandling<T> for Vec<RangeInclusive<T>>
where
    T: Copy + PartialEq + PartialOrd + Num,
{
    fn deduplicate_ranges(&self) -> Vec<RangeInclusive<T>> {
        let mut new_ranges = Vec::new();

        for range in self.iter() {
            let mut cur_range = VecDeque::new();
            cur_range.push_back(range.clone());

            'outer: while let Some(value) = cur_range.pop_front() {
                for new_range in new_ranges.clone().into_iter() {
                    if value == new_range
                        || value.start() >= new_range.start() && value.end() <= new_range.end()
                    {
                        continue 'outer;
                    }

                    if value.start() >= new_range.start()
                        && value.end() >= new_range.end()
                        && value.start() <= new_range.end()
                    {
                        cur_range.push_back(*new_range.end() + T::one()..=*value.end());
                        continue 'outer;
                    }
                    if value.start() <= new_range.start() && value.end() > new_range.end() {
                        cur_range.push_back(*value.start()..=*new_range.start() - T::one());
                        cur_range.push_back(*new_range.end() + T::one()..=*value.end());
                        continue 'outer;
                    }
                    if value.start() <= new_range.start() && value.end() >= new_range.start() {
                        cur_range.push_back(*value.start()..=*new_range.start() - T::one());
                        continue 'outer;
                    }
                }

                new_ranges.push(value);
            }
        }

        new_ranges
    }
}
