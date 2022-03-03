/*
    佇列：先進先出、後進後出

    改善【0002_佇列】字串的clone()的效能問題
    benchmark(push、next各10000次): time: [6.3894 ms 6.4243 ms 6.4591 ms]
*/

use std::rc::Rc;

use code::Personal;
use code::PersonalAry;

fn main() {
    let mut personal_ary = PersonalAry {
        personal: Vec::new()
    };

    personal_ary.personal.push(Rc::new(Personal { name: String::from("A"), age: 52 }));
    personal_ary.personal.push(Rc::new(Personal { name: String::from("B"), age: 23 }));
    personal_ary.personal.push(Rc::new(Personal { name: String::from("C"), age: 35 }));

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
