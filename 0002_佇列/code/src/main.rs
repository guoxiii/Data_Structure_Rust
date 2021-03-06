/*
    佇列：先進先出、後進後出

    字串的clone()會有效能上的問題，可改用另一個版本【0003_佇列_智能指針版】
    benchmark(push、next各10000次): time: [22.476 ms 22.541 ms 22.608 ms]
*/

use code::Personal;
use code::PersonalAry;

fn main() {
    let mut personal_ary = PersonalAry {
        personal: Vec::new()
    };

    personal_ary.personal.push(Personal { name: String::from("A"), age: 52 });
    personal_ary.personal.push(Personal { name: String::from("B"), age: 23 });
    personal_ary.personal.push(Personal { name: String::from("C"), age: 35 });

    // PersonalAry { personal: [Personal { name: "A", age: 52 }, Personal { name: "B", age: 23 }, Personal { name: "C", age: 35 }] }
    println!("{:?}", personal_ary);

    // Some(Personal { name: "A", age: 52 })
    println!("{:?}", personal_ary.next());

    // PersonalAry { personal: [Personal { name: "B", age: 23 }, Personal { name: "C", age: 35 }] }
    println!("{:?}", personal_ary);

    // Some(Personal { name: "B", age: 23 })
    println!("{:?}", personal_ary.next());

    // Some(Personal { name: "C", age: 35 })
    println!("{:?}", personal_ary.next());

    // None
    println!("{:?}", personal_ary.next());
}
