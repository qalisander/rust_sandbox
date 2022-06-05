use itertools::Itertools;
use num::{Complex, Float};
use std::cmp::Ordering;
//use ordered_float::OrderedFloat;
use std::collections::{BTreeMap, VecDeque};
use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
struct OrdFloat<T: Float>(T);

impl<T: Float> Eq for OrdFloat<T> {}

#[allow(clippy::derive_ord_xor_partial_ord)]
impl<T: Float + Debug> Ord for OrdFloat<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other)
            .expect(&*format!("Invalid points! {self:?} and {other:?}"))
    }
}

impl<T: Float> From<T> for OrdFloat<T> {
    fn from(f: T) -> Self {
        Self(f)
    }
}

fn closest_pair(points: &[(f64, f64)]) -> ((f64, f64), (f64, f64)) {
    let points = points
        .iter()
        .sorted_by(|l, r| l.0.partial_cmp(&r.0).unwrap())
        .map(|point| Complex::new(point.0, point.1))
        .collect_vec();

    let mut within_distance_tree: BTreeMap<OrdFloat<f64>, Vec<Complex<f64>>> = BTreeMap::new();
    let mut within_distance_deque: VecDeque<Complex<f64>> = VecDeque::new();
    let mut min_distance = f64::max_value();
    let mut closest_pair: Option<(Complex<f64>, Complex<f64>)> = None;
    for point in points {
        while let Some(front_point) = within_distance_deque.pop_front() {
            if point.re - front_point.re > min_distance {
                within_distance_tree.remove(&front_point.im.into());
            } else {
                within_distance_deque.push_front(front_point);
                break;
            }
        }
        let from: OrdFloat<f64> = (point.im - min_distance).into();
        let to: OrdFloat<f64> = (point.im + min_distance).into();
        for last_point in within_distance_tree
            .range(from..=to)
            .flat_map(|(im, re_vec)| re_vec)
        {
            let new_distance = (point - last_point).norm();
            if min_distance > new_distance {
                closest_pair = Some((point, *last_point));
                min_distance = new_distance;
            }
        }

        within_distance_deque.push_back(point);
        within_distance_tree
            .entry(point.im.into())
            .or_default()
            .push(point)
    }

    let (p0, p1) = closest_pair.expect("Closest pair was not found!");
    ((p0.re, p0.im), (p1.re, p1.im))
}

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
        verify(
            closest_pair(&[
                (0.8998374006766753, 0.043522294993519575),
                (0.8216028049856874, -0.12236318141421343),
                (0.8404749093035422, 0.14150986215607433),
                (0.68412054221526, -0.05960435780294515),
                (0.9072899317749884, -0.1213320990315292),
                (0.8680109777054663, -0.011004586025835816),
                (0.955564922497625, -0.015255502666314591),
                (0.9095182633279054, -0.0472114610598077),
                (0.7429155700062877, 0.06450600858898675),
                (0.7985128898953212, 0.09487536896197707),
                (0.9967678049453048, -0.04311446994516366),
                (0.9498340450342942, -0.08221834308291104),
                (0.7703868887962588, -0.06844547178125177),
                (0.6865986612274764, 0.022998899629499955),
                (0.8079205484329129, -0.2025073420691922),
                (0.7840591810846355, 0.004583313102512865),
                (0.7688851770861618, -0.1470419009761582),
                (0.8184812921141641, -0.03722683106304919),
                (0.8597824873927521, -0.10281969003377661),
                (0.8628729397138784, 0.09725904195528501),
                (0.6601486780399023, -0.02337303278844652),
                (0.8402555703314102, -0.171823914407645),
                (0.7236405605138294, -0.09801689452850854),
                (0.8296038385996926, 0.04667935706242876),
                (0.7376120836945961, 0.0685205074962138),
                (0.7527932749652894, -0.030321327742144577),
            ]),
            (
                (0.7376120836945961, 0.0685205074962138),
                (0.7429155700062877, 0.06450600858898675),
            ),
        )
    }
}
