fn main() {
    // String: luu o trong heap
    // UTF-8
    //
    // De ta thay duoc no la 1 chuoi cac byte dung canh nhau
    let s2 = String::from("");
    for i in s2.bytes() {
        println!("{}", i);
    }
}
