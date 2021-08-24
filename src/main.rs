/*

. String - owned
. &str - borrowed String slice

Must use an owned String to store in a struct
Use &str when passing to a function

 */

/*
Here's an example of a struct, and we're attempting to store a string slice within the struct,
just mentioned, when you create a string in line 2nd after main() "Nhatnv", it's automatically borrowed
If we try to store borrowed data and structure, it won't compile
And the reason is when this structure (Employee) is to be dropped at the end of the scope, the structure is responsible for
cleaning up its own memory. However, since we have borrowed memory (&str), the structure is not allowed to clean it up because it doesn't own this data
So it results in a compiler error and it'll inform you that you cannot store a string slice in this
 */
struct Employee {
    name: &str,
}

fn main(){
    let emp_name = "Nhatnv";
    let emp = Employee {
        name: emp_name
    };
}
