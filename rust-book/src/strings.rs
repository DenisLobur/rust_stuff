pub fn using_strings() {
    let mut str = String::new();
    println!("Empty string: {:?}", str);

    let data = "initial contents";
    let s = data.to_string();
    println!("String from data: {:?}", s);
    println!("String from data2: {:?}", "initial contents".to_string());

    let mut s = String::from("initial contents");
    s.push_str(" and more");
    s.push('!');
    println!("String from data3: {:?}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = format!("{s1}{s2}");
    println!("String from data4: {:?}", s3);

    for c in s3.chars() {
        println!("{c}");
    }
}