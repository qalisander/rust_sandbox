use itertools::Itertools;
use ordered_float::{Float, OrderedFloat};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::ops::RangeInclusive;
use num::pow::Pow;

// TODO: use ord float just for indexing sorted collection
type OrdFloat<T> = OrderedFloat<T>;

fn closest_pair(points: &[(f64, f64)]) -> ((f64, f64), (f64, f64)) {
    let points = points
        .iter()
        .map(|point| (OrdFloat::from(point.0), OrdFloat::from(point.1)))
        .sorted()
        .collect_vec();
    // NOTE: sorted by last coordinate
    let mut points_within_distance: BTreeSet<(OrdFloat<f64>, OrdFloat<f64>)> = BTreeSet::new();
//    let mut points_within_distance_map: BTreeMap<OrdFloat<f64>, Vec<(f64, f64)>> = BTreeMap::new();
    let mut deque: VecDeque<(OrdFloat<f64>, OrdFloat<f64>)> = VecDeque::new();
    let distance = OrdFloat::max_value();
    let mut closest_pair: Option<((f64, f64), (f64, f64))> = None;
    for point in points {
        if let Some(front_point) = deque.pop_front() {
            if point.0 - front_point.0 > distance {
                points_within_distance.remove(&(front_point.1, front_point.0));
            } else {
                deque.push_front(front_point)
            }
        }
        let (min, max) = (OrdFloat::min_value(), OrdFloat::max_value());
        for &(point1, point0) in
            points_within_distance.range(&(point.1 - distance, min)..&(point.1 + distance, max))
        {
            let new_distance = distance_between((point0.into(), point1.into()), (point0.into(), point1.into()));
            if distance > new_distance {
                closest_pair.insert(((point0, point1), point));
                distance = new_distance;
            }
        }

        deque.push_back(point)
    }

    return closest_pair.expect("Closest pair was not found!")
}

fn distance_between(left: (OrdFloat<f64>, OrdFloat<f64>), right: (OrdFloat<f64>, OrdFloat<f64>)) -> OrdFloat<f64>{
    ((right.0 - left.0).pow(2_f64) + (right.1 - left.1).pow(2_f64)).pow(0.5_f64)
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::closest_pair;

    type Points = ((f64, f64), (f64, f64));

    fn verify(actual: Points, expected: Points) {
        if actual == expected || (actual.0 == expected.1 && actual.1 == expected.0) {
            assert!(true)
        } else {
            assert_eq!(actual, expected)
        }
    }

    #[test]
    fn sample_tests() {
        verify(
            closest_pair(&[(2.0, 2.0), (6.0, 3.0)]),
            ((2.0, 2.0), (6.0, 3.0)),
        );
        verify(
            closest_pair(&[
                (2.0, 2.0),
                (2.0, 8.0),
                (5.0, 5.0),
                (6.0, 3.0),
                (6.0, 7.0),
                (7.0, 4.0),
                (7.0, 9.0),
            ]),
            ((6.0, 3.0), (7.0, 4.0)),
        );
        verify(
            closest_pair(&[
                (2.0, 2.0),
                (2.0, 8.0),
                (5.0, 5.0),
                (5.0, 5.0),
                (6.0, 3.0),
                (6.0, 7.0),
                (7.0, 4.0),
                (7.0, 9.0),
            ]),
            ((5.0, 5.0), (5.0, 5.0)),
        );
    }
}
