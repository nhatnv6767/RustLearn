use std::io;
// just need to provide the successful data type
fn get_input() -> io::Result<String>{
    // buffer: some other functionality can use and operate with
    let mut buffer = String::new();
    // it reads one line of input from the standard input
    // read a line and then save that line into this buffer that is borrowing
    // the ?: which indicates to us that this function may possibly fail
    // so if this fails, we'll return an error from the function
    // and the data will not have been read from the terminal
    // if the data is read properly, then it will be automatically available within our buffer
    // so to get the data from the buffer into our program, all we need to do is just return it
    io::stdin().read_line(&mut buffer)?;
    // when press enter the enter is actually included as part of the data
    // trim simply trims any whitespace
    // to_owned because when we do trim, it just creates a slice
    // because the data is all there and it only needs to return the data
    // that is not empty spaces. we can return a string properly
    Ok(buffer.trim().to_owned())
}

fn main() {
    let mut all_input = vec![];
    let mut times_input = 0;
    while times_input < 2 {
        match get_input() {
            // word user type
            Ok(words) => {
                all_input.push(words);
                times_input += 1;
            }
            Err(e) => println!("error: {}", e),
        }
    }

    for input in all_input {
        println!("Original: {}, capitalized: {}", input, input.to_uppercase());
    }
}
