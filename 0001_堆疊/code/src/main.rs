/*
堆疊：先進後出、後進先出
*/

#[derive(Debug)]
struct Personal {
    name: String,
    age: u32
}

fn main() {
    let mut ps: Vec<Personal> = Vec::new();

    ps.push(Personal { name: String::from("A"), age: 52 });
    ps.push(Personal { name: String::from("B"), age: 52 });
    ps.push(Personal { name: String::from("C"), age: 52 });

    // [Personal { name: "A", age: 52 }, Personal { name: "B", age: 52 }, Personal { name: "C", age: 52 }]
    println!("{:?}", ps);

    // Some(Personal { name: "C", age: 52 })
    println!("{:?}", ps.pop());

    // Some(Personal { name: "B", age: 52 })
    println!("{:?}", ps.pop());

    // Some(Personal { name: "A", age: 52 })
    println!("{:?}", ps.pop());

    // None
    println!("{:?}", ps.pop());

    // []
    println!("{:?}", ps);    
}
