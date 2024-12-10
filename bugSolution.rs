fn main() {
    let mut v = vec![1, 2, 3];
    // Correct way to modify a vector
    v[0] = 4; 
    println!("{:?}", v);

    //Alternative approach without raw pointers if needed
    //let mut v = vec![1, 2, 3];
    //v.iter_mut().for_each(|x| *x *= 2); //Multiplies each element by 2
    //println!("{:?}", v);
} 