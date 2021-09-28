use unicode_segementation::UnicodeSegementation;
fn main() {
    // String: luu o trong heap
    // UTF-8
    //
    // De ta thay duoc no la 1 chuoi cac byte dung canh nhau
    let s2 = String::from("hkli");

    // Bytes: Kieu luu tru Bytes
    for i in s2.bytes() {
        println!("{}", i);
    }

    // Scalar value: Kieu vo huong, luu tru nhung ky tu ma no duoc nam rieng ra
    // tung byte tung byte khac nhau
    for i in s2.chars() {
        println!("{}", i);
    }

    // Grapheme Clusters
    // 1 so ngon ngu dac biet nhu cua Hindu, khi in ra chars se xuat hien 1 so ky tu la
    // va no duoc luu o nhung byte dac biet khac, nen can kieu grapheme nay
    // dung unicode de in ra dong chu hoan chinh
    for i in s2.graphemes(true) {
        println!("{}", i)
    }
}
