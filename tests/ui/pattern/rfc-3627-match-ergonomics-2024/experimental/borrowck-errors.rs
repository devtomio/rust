//@ edition: 2024
//@ revisions: classic structural
//! Tests for pattern errors not handled by the pattern typing rules, but by borrowck.
#![allow(incomplete_features)]
#![cfg_attr(classic, feature(ref_pat_eat_one_layer_2024))]
#![cfg_attr(structural, feature(ref_pat_eat_one_layer_2024_structural))]

pub fn main() {
    if let Some(&Some(x)) = Some(&Some(&mut 0)) {
        //~^ ERROR: cannot move out of a shared reference [E0507]
        let _: &u32 = x;
    }

    let &ref mut x = &0;
    //~^ cannot borrow data in a `&` reference as mutable [E0596]

    if let &Some(Some(x)) = &Some(&mut Some(0)) {
        //[classic]~^ ERROR: cannot borrow data in a `&` reference as mutable
        let _: &u32 = x;
    }

    let &[x] = &&mut [0];
    //[classic]~^ ERROR: cannot borrow data in a `&` reference as mutable
    let _: &u32 = x;

    let [&x] = &[&mut 0];
    //[classic]~^ ERROR: cannot move out of type
    let _: &u32 = x;

    #[cfg(classic)] // TODO: this should pass on `structural` but doesn't
    let [&x] = &mut [&mut 0]; //[classic]~ ERROR: cannot move out of type
    let _: &u32 = x;

    let [&mut x] = &mut [&mut 0];
    //[classic]~^ ERROR: cannot move out of type
    let _: &mut u32 = x;
}
