// Reference pointer - references to a resource in memory

pub fn run() {
    // Primitive array
    let a1 = [1,2,3];
    let a2 = a1;

    println!("{:?}", (a1, a2));

    // With non-primitives (Vectors), if you assign another variable to a piece of data,
    // the first variable will no longer hold that value.
    // You'll need to use a reference (&) to point to the resource.

    // Vector
    let v1 = vec![1,2,3];
    let v2 = &v1;

    println!("{:?}", (&v1, v2));

}