fn main() {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);


    for i in &mut v {
        *i += 50;
    }
    
    println!("{:?}", v);
}
