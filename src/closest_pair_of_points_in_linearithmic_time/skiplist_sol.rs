use itertools::Itertools;
use num::{Complex, Float};
use std::cmp::Ordering;
use skiplist::{OrderedSkipList, SkipMap};
use std::fmt::Debug;
use std::ops::Bound::Included;

#[derive(Debug, Clone, Copy)]
struct OrdByLastFloatContainer<T: Float>(T, T);

impl<T: Float> OrdByLastFloatContainer<T> {
    fn norm(&self, other: &OrdByLastFloatContainer<T>) -> T {
        ((self.0 - other.0).powi(2) + (self.1 - other.1).powi(2)).sqrt()
    }
}

impl<T: Float> PartialEq for OrdByLastFloatContainer<T> {
    fn eq(&self, other: &Self) -> bool {
        self.1.eq(&other.1)
    }
}

impl<T: Float> PartialOrd for OrdByLastFloatContainer<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.1.partial_cmp(&other.1)
    }
}

impl<T: Float> Eq for OrdByLastFloatContainer<T> {}

#[allow(clippy::derive_ord_xor_partial_ord)]
impl<T: Float> Ord for OrdByLastFloatContainer<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<T: Float + Default + Debug> From<T> for OrdByLastFloatContainer<T> {
    fn from(t: T) -> Self {
        Self(T::default(), t)
    }
}

pub(super) fn closest_pair(points: &[(f64, f64)]) -> ((f64, f64), (f64, f64)) {
    let points_sorted_by_first = points
        .iter()
        .sorted_by(|l, r| l.0.partial_cmp(&r.0).unwrap())
        .map(|point| OrdByLastFloatContainer(point.0, point.1))
        .collect_vec();

    let mut within_distance_tree: OrderedSkipList<OrdByLastFloatContainer<f64>> =
        OrderedSkipList::new();
    let mut last_within_distance_index = 0;
    let mut min_distance = f64::max_value();
    let mut closest_pair: Option<(OrdByLastFloatContainer<f64>, OrdByLastFloatContainer<f64>)> =
        None;
    for point in &points_sorted_by_first {
        while let Some(last_point) = points_sorted_by_first.get(last_within_distance_index) {
            if point.0 - last_point.0 > min_distance {
                within_distance_tree.remove(&last_point.0.into());
                last_within_distance_index += 1;
            } else {
                break;
            }
        }

        let from: OrdByLastFloatContainer<f64> = (point.1 - min_distance).into();
        let to: OrdByLastFloatContainer<f64> = (point.1 + min_distance).into();
        for last_point in within_distance_tree.range(Included(&from), Included(&to)) {
            if last_point.1 - point.1 >= min_distance {
                break;
            }

            let new_distance = point.norm(last_point);
            if min_distance > new_distance {
                closest_pair = Some((*point, *last_point));
                min_distance = new_distance;
            }
        }

        within_distance_tree.insert(*point)
    }

    let (p0, p1) = closest_pair.expect("Closest pair was not found!");
    ((p0.0, p0.1), (p1.0, p1.1))
}