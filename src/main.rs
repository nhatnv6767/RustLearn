use std::collections::HashMap;

fn main() {
    // Hashmap
    let text = "hello world this is wonderful world";
    let mut map = HashMap::new();
    // dau tien no se xoa nhung khoang trang
    // sau do no se duyet qua cac phan tu (chu cai) o trong text do
    // neu chua co gia tri thi no se insert 0 vao map
    // sau khi chay xong thi no se tang count len 1
    // va khi da chay het (tat ca deu co gia tri count = 1) no lai gap thang world 1 lan nua,
    // no se + 1 them 1 lan nua vao, va khi do ta co world = 2
    for i in text.split_whitespace() {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
