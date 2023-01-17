// Point to a resource in memeory

pub fn run() {
    // Primitive array
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    println!("Values: {:?}", (arr1, arr2));

    // Vector
    let vec1 = vec![1, 2, 3];

    // vec1 will no longer hold the value. This happens with all non-prims
    let _vec2 = vec1;

    // This line will give an error because vec1 has been moved
    // println!("Values: {:?}", (vec1, vec2));

    // Instead, use a reference (&)
    let vec3 = vec![4, 5, 6];
    let vec4 = &vec3;
    println!("Values: {:?}", (&vec3, vec4));
}
