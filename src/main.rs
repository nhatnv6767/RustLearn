// Option Combinators
// allow to easily manipulate data and manage options
//
fn main() {
    let a: Option<i32> = None;
    dbg!(a);
    // will check to see if we have some data within our option
    let a_is_some = a.is_some();
    dbg!(a_is_some);
    let a_is_none = a.is_none();
    dbg!(a_is_none);
    // map allow closure
    // if a has some data -> will do this, else will don't
    let a_mapped = a.map(|num| num + 1);
    dbg!(a_mapped);
    // filter combinator: similar to map
    // it takes a closure that accepts a single function argument
    // the body need to return, either true or false
    // if the return value is true, then we'll keep the optional data
    // and at the return value is false, we'll throw away the optional data
    // if you already have no data, then this filter won't do anything at all
    // because there's already no data present
    let a_filtered = a.filter(|num| num == &1);
    dbg!(a_filtered);
    // or_else accept closure
    // this particular closure does not use any arguments,
    // so we can just use || to signify no arguments
    // can return nothing or something
    // explain: this will do if a is already data, then nothing's going to happen
    // if a is no data, then we're going to return some new data. -> will return option data (some or none)
    let a_or_else = a.or_else(|| Some(5));
    dbg!(a_or_else);
    // accepts a closure that takes no arguments similar or else then we can return a default value
    //will actually take out the data and then place it within this variable
    // this will no longer have optional data
    // if we started with some optional data and then called UNWRASP... the original data will be placed
    // if no data, then when we call this function, this default data will be saved
    let unwrapped = a.unwrap_or_else(|| 0);
    dbg!(unwrapped);
}
