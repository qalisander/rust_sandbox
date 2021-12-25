use bit_set::BitSet;
use bit_vec::BitVec;
use core::iter;
use itertools::{Itertools, PeekingNext};
use num::Complex;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::ops::Index;
//use rust_sandbox::nonogram_solvers::nonogram_solver_slice::solve_nonogram;
//use rust_sandbox::nonogram_solvers::test::{CLUES_15, print};


fn main() {
    use std::rc::{Rc, Weak};

    let strong = Rc::new("hello".to_owned());

    let raw_1 = Rc::downgrade(&strong).into_raw();
    let raw_2 = Rc::downgrade(&strong).into_raw();
    let weak = Rc::downgrade(&strong);
    let i = weak.upgrade().unwrap().len();


    assert_eq!(2, Rc::weak_count(&strong));

    assert_eq!("hello", &*unsafe { Weak::from_raw(raw_1) }.upgrade().unwrap());
    assert_eq!(1, Rc::weak_count(&strong));

    drop(strong);

// Decrement the last weak count.
    assert!(unsafe { Weak::from_raw(raw_2) }.upgrade().is_none());

