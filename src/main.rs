// Iterators are a simple way to traverse and manipulate data structure
//
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // let mut plus_one = vec![];
    // for num in numbers {
    //     plus_one.push(num + 1);
    // }

    // create a new iterator
    let plus_one: Vec<_> = numbers
        .iter()
        .map(|num| num + 1)
        // collect simply creates a new vector out of this iterator
        .collect();

    let new_numbers: Vec<_> = numbers
        .iter()
    // filter only keep the ones you want
    // if the number is less than or equal to 3, we will keep it with
    // filters whenever you return true for a particular number, then
    // we'll keep that value and if we return false, it will remove that
        .filter(|num| num <= 3)
        .collect();

    let numbers = vec![1, 2, 3, 4, 5];
    // Optional value because there is the possibility of this number (3) not being in
    // this vector (vector numbers)
    let find_me: Option<i32> = numbers
        .iter()
        .find(|num| num == 3);

    // return the number of elements within the iterator
    let count = numbers
        .iter()
        .count();

    // return the last element in an iterator
    // Option: it is possible to create an empty vector
    // ex: if we remove all the item within the vector and then
    // try to get the last one, there's nothing to return.
    // So that's why this one returns an option when working with numbers (vector numbers)

    let last: Option<i32> = numbers
        .iter()
        .last();
    // it also have min, max, use like last()
    //

    // it takes some items from the vector and then that's the only one
    // you'll have.
    let take: Vec<i32> = numbers
        .iter()
        // it will take the first, 2nd, 3rd and others will not
        .take(3)
        .collect();
}

// if we wanted to create another vector with all of these numbers
// but add one to each number



// explain about iterator: The reason we need to use type annotations in this case
// is because iter and collect operate generically on any kind of data
// structures so we can work on hash maps,
// vectors and other kind of data structure


// an iterator as a plan telling program they want to take the numbers vectors
// we're planning on going through each item (iter) in the vector
// And we do go through each item, -> we want to map each item by taking the item
// adding one to it. And then finally, when we're done, we want to collect it
// And again, the collection will collect the items into a new vector
// since that's what we specify in the type annotations (Vec<_>)
