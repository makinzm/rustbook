fn main() {
    let mut v1 = vec![1,2,3];
    // let mut v2 = v1;
    let mut v2 = v1.clone();
    v1[1] = 100;
    //  value borrowed here after move -> we should use clone.
    for v in &v1{
        println!("v1:{}",v);
    }
    // help: consider iterating over a slice of the `Vec<i32>`'s content to avoid moving into the `for` loop
    for v in &v2{
        println!("v2:{}",v);
    }

    println!();

    v2[1] = -100;
    for v in &v1{
        println!("v1:{}",v);
    }
    for v in &v2{
        println!("v2:{}",v);
    }
}
