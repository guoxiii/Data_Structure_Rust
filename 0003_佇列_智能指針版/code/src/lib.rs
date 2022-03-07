use std::rc::Rc;

#[derive(Debug)]
pub struct Personal {
    pub name: String,
    pub age: u32
}

#[derive(Debug)]
pub struct PersonalAry {
    pub personal: Vec<Rc<Personal>>
}

impl Iterator for PersonalAry {
    type Item = Rc<Personal>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.personal.first() {
            Some(p) => {
                let personal = Rc::clone(p);
                self.personal.remove(0);
                Some(personal)
            },
            None => None
        }
    }
}
