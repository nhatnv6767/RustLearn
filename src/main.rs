/*

. String - owned
. &str - borrowed String slice

Must use an owned String to store in a struct
Use &str when passing to a function

Summary
. Strings are automatically borrowed
. Use .to_owned() or String::from() to create an owned copy of a string slice
. Use an owned String when storing in a struct

 */

struct Employee {
    name: String,
}

fn main() {
    // 2 line the same meaning, that will create owned data and then the own data is transferred into the
    // employee structure. So the employee then owns that name
    // And when structure eventually gets dropped near the end of the program or wherever it happens to be dropped,
    // it will be allowed to clear this memory and therefore the compiler will succeed
    let emp_name = "Nhatnv".to_owned();
    let emp_name = String::from("Nhatnv");
    let emp = Employee {
        name: emp_name
    };
}
