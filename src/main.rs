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
