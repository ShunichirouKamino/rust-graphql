fn main() {
    let list: Vec<&str> = "hello,world,goodbye,world".split(',').collect();
    let elem = list.get(0).unwrap();

    println!("{}", elem);
}
