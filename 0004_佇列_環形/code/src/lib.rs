use std::rc::{ Rc, Weak };

#[derive(Debug)]
pub struct Personal {
    pub name: String,
    pub age: u32
}

#[derive(Debug)]
pub struct PersonalAry {
    front: usize,
    rear: usize,
    count: usize,
    max: usize,
    personal: Vec<Rc<Personal>>
}

impl PersonalAry {
    pub fn new(max: usize) -> PersonalAry {
        PersonalAry {
            front: 0,
            rear: 0,
            count: 0,
            max,
            personal: Vec::new()
        }
    }

    pub fn push(&mut self, p: Personal) -> Result<usize, &str> {
        if self.count == self.max {
            return Err("佇列已滿")
        }

        if self.personal.len() < self.max {
            self.personal.push(Rc::new(p));
        } else {
            self.personal[self.rear] = Rc::new(p);
        }

        self.rear += 1;
        self.rear %= self.max;

        self.count += 1;

        Ok(self.count)
    }
}

impl Iterator for PersonalAry {
    type Item = Weak<Personal>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.count {
            0 => None,
            _ => {
                let p = self.personal.get(self.front);
                self.count -= 1;
                self.front += 1;
                self.front %= self.max;
                Some(Rc::downgrade(p.unwrap()))
            }
        }
    }
}
