fn main() {
    let mut v = vec![1, 2, 3];
    let index = 0;
    v[index] = 10; //Safe way to modify vector elements
    println!("v: {:?}", v);
}
