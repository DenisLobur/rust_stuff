pub fn give_me_vector() {
    let v: Vec<i32> = Vec::new();
    println!("Vector: {:?}", v);
    let v2 = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v2[2];
    println!("Third element: {third}");
    let third_option: Option<&i32> = v2.get(2);
    match third_option {
        Some(third) => println!("Third option element: {third}"),
        None => println!("No third option element"),
    }

    // println!("Does not exist1: {:#?}", v2[100]);
    // println!("Does not exist2: {:#?}", v2.get(100));
    println!("Vector2: {:?}", v2);
    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);
    println!("Vector3: {:?}", v3);

    println!("Iterating over immutable vector: ");
    for i in &v2 {
        println!("{i}");
    }

    println!("Iterating over mutable vector: ");
    for i in &mut v3 {
        *i += 50;
        println!("{i}");
    }
}