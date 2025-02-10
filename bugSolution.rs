fn main() {
    let mut x = 5;
    { //Create a new scope for the first mutable reference
        let y = &mut x;
        *y += 1;
    }
    { //Create a new scope for the second mutable reference
        let z = &mut x;
        *z += 1;    
    }
    println!("{}", x);
}