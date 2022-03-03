/*
    佇列：先進先出、後進後出
*/

#[derive(Debug)]
struct Personal {
    name: String,
    age: u32
}

impl Clone for Personal {
    fn clone(&self) -> Personal {
        Personal {
            name: self.name.clone(),
            age: self.age
        }
    }
}

#[derive(Debug)]
struct PersonalAry {
    personal: Vec<Personal>
}

impl Iterator for PersonalAry {
    type Item = Personal;

    fn next(&mut self) -> Option<Self::Item> {
        match self.personal.first() {
            Some(p) => {
                let personal = p.clone();
                self.personal.remove(0);
                Some(personal)
            },
            None => None
        }
    }
}

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
