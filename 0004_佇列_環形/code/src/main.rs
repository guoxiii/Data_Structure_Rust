/*
    佇列：先進先出、後進後出
    此範例為環形佇列
*/

use code::Personal;
use code::PersonalAry;

fn main() {
    let mut personal_ary = PersonalAry::new(10);

    personal_ary.push(Personal { name: String::from("A"), age: 52 });
    personal_ary.push(Personal { name: String::from("B"), age: 23 });
    personal_ary.push(Personal { name: String::from("C"), age: 35 });

    // PersonalAry { front: 0, rear: 3, count: 3, max: 10, personal: [Personal { name: "A", age: 52 }, Personal { name: "B", age: 23 }, Personal { name: "C", age: 35 }] }
    println!("{:?}", personal_ary);
    
    // Some(Personal { name: "A", age: 52 })
    println!("{:?}", personal_ary.next().unwrap().upgrade());

    // PersonalAry { front: 1, rear: 3, count: 2, max: 10, personal: [Personal { name: "A", age: 52 }, Personal { name: "B", age: 23 }, Personal { name: "C", age: 35 }] }
    println!("{:?}", personal_ary);

    // Some(Personal { name: "B", age: 23 })
    println!("{:?}", personal_ary.next().unwrap().upgrade());

    // Some(Personal { name: "C", age: 35 })
    println!("{:?}", personal_ary.next().unwrap().upgrade());

    // PersonalAry { front: 3, rear: 3, count: 0, max: 10, personal: [Personal { name: "A", age: 52 }, Personal { name: "B", age: 23 }, Personal { name: "C", age: 35 }] }
    println!("{:?}", personal_ary);

    for x in 0 .. 15 {
        personal_ary.push(Personal { name: String::from(format!("{}", x)), age: x });
    }

    /*
    // 最多10個物件(使用new時設定)
    Some(Personal { name: "0", age: 0 })
    Some(Personal { name: "1", age: 1 })
    Some(Personal { name: "2", age: 2 })
    Some(Personal { name: "3", age: 3 })
    Some(Personal { name: "4", age: 4 })
    Some(Personal { name: "5", age: 5 })
    Some(Personal { name: "6", age: 6 })
    Some(Personal { name: "7", age: 7 })
    Some(Personal { name: "8", age: 8 })
    Some(Personal { name: "9", age: 9 })
    */
    for p in personal_ary {
        println!("{:?}", p.upgrade());
    }

    let mut personal_ary = PersonalAry::new(5);

    /*
    Result = 1
    Result = 2
    Result = 3
    Result = 4
    Result = 5
    Error  = 佇列已滿
    Error  = 佇列已滿
    Error  = 佇列已滿
    */
    for x in 0 .. 8 {
        match personal_ary.push(Personal { name: String::from(format!("{}", x)), age: x }) {
            Ok(v) => println!("Result = {}", v),
            Err(e) => println!("Error  = {}", e)
        }
    }
}
