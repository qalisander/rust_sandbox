//https://www.codewars.com/kata/5376b901424ed4f8c20002b7
use itertools::Itertools;
use num::{Complex, Float};
use std::cmp::Ordering;
use std::collections::{BTreeMap, VecDeque};
use std::f64;
use std::fmt::Debug;
use std::ops::{Add, AddAssign};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
struct OrdFloat<T: Float>(T);

impl<T: Float> Eq for OrdFloat<T> {}

#[allow(clippy::derive_ord_xor_partial_ord)]
impl<T: Float + Debug> Ord for OrdFloat<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other)
            .unwrap_or_else(|| panic!("Invalid points! {self:?} and {other:?}"))
    }
}

impl<T: Float> From<T> for OrdFloat<T> {
    fn from(f: T) -> Self {
        Self(f)
    }
}

pub(super) fn closest_pair(points: &[(f64, f64)]) -> ((f64, f64), (f64, f64)) {
    let points = points
        .iter()
        .sorted_by(|l, r| l.0.partial_cmp(&r.0).unwrap())
        .map(|point| Complex::new(point.0, point.1))
        .collect_vec();

    let mut within_distance_tree: BTreeMap<OrdFloat<f64>, Complex<f64>> = BTreeMap::new();
    let mut last_within_distance_index = 0;
    let mut min_distance = f64::max_value();
    let mut closest_pair: Option<(Complex<f64>, Complex<f64>)> = None;
    for &point in &points {
        while let Some(last_point) = points.get(last_within_distance_index) {
            if point.re - last_point.re > min_distance {
                within_distance_tree.remove(&last_point.im.into());
                last_within_distance_index += 1;
            } else {
                break;
            }
        }
        let from: OrdFloat<f64> = (point.im - min_distance).into();
        let to: OrdFloat<f64> = (point.im + min_distance).into();
        for (_, &last_point) in within_distance_tree.range(from..=to) {
            if last_point.im - point.im >= min_distance {
                break;
            }

            let new_distance = (point - last_point).norm();
            if min_distance > new_distance {
                closest_pair = Some((point, last_point));
                min_distance = new_distance;
            }
        }

        let mut key: OrdFloat<f64> = point.im.into();
        let mut point_to_insert = point;
        while let Some(replaced_point) = within_distance_tree.insert(key, point_to_insert) {
            key.0 += 0.00000000001;
            point_to_insert = replaced_point;
        }
    }

    let (p0, p1) = closest_pair.expect("Closest pair was not found!");
    ((p0.re, p0.im), (p1.re, p1.im))
}
