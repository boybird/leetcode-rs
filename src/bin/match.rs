fn main() {
    // match 匹配
    let t = (42, "I Love Rust".to_string());
    let (n, s) = t;
    println!("{:?} {:?}", n, s);

    // Ref 借用
    let t2 = (42, "I Love Rust".to_string());
    let (ref n2, ref s2) = t2;
    println!("{:?} {:?}", n2, s2);
    // Ref 借用2
    let ot = Some((42, "Hello World".to_string()));
    if let Some((_, ref s)) = ot {
        assert_eq!(s, "Hello World");
    }
    // match_tuple_0(t);
    match_tuple_1(&t2);
    match_tuple_0(t2);
    // match_tuple_1(&t2);
    match_tuple_1(&(1, "hello".to_string()));

    let mut rebote_name = Some(String::from("Bors"));
    match rebote_name {
        Some(ref mut name) => *name = String::from("Another name"),
        None => (),
    };
    println!("{:?}", rebote_name);

    match_tuple_2((0, "hello".to_string()));
}

fn match_tuple_2(t: (i32, String)) {
    let text = match t {
        // (0, s) => format!("zero {}", s),
        (0, ref s) if s == "hello" => format!("hello one"),
        tt => format!("{:?}", tt),
    };
    println!("{}", text);
}

fn match_tuple_0(t: (i32, String)) {
    let text = match t {
        (0, s) => format!("zero {}", s),
        (1, ref s) if s == "hello" => format!("hello one!"),
        tt => format!("no match {:?}", tt),
        // or say _ => format!("no match") if you're not interested in the value
    };
    println!("{}", text);
}

fn match_tuple_1(t: &(i32, String)) {
    let text = match t {
        (0, ref s) => format!("zero {}", s),
        (1, ref s) if s == "hello" => format!("hello one!"),
        ref tt => format!("no match {:?}", tt),
    };
    println!("{}", text);
}
