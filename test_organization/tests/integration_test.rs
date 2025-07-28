use test_organization::add_two;
mod common; //Declaring a module common. This way we are using common

// no need o add #[cfg(test)] here!
// cargo threats test directory as a collection of test code by default

#[test]
fn it_adds_two() {
    common::setup(); // using common
    let result = add_two(2);
    assert_eq!(result, 4);
}

// It we would like to run only this file during testing:
//   > cargo test --test integration_test