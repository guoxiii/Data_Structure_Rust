use std::collections::HashMap;

struct OperPriority {
    ch: char,       // 運算子、運算元
    priority: i32   // 優先權，數字越大優先權越高
}

impl Clone for OperPriority {
    fn clone(&self) -> OperPriority {
        OperPriority {
            ch: self.ch,
            priority: self.priority
        }
    }
}

pub struct Priority {
    pos_neg: bool,                  // true: +、-是正負符號， false: +、-是加減符號
    stack: Vec<OperPriority>,       // 儲存各運算元和運算子與優先權
    in_coming: HashMap<char, i32>,  // 設定運算子的優先權
}

impl Priority {
    pub fn new(expr: &str) -> Priority {
        let mut priority = Priority {
            pos_neg: true,              // 預設+、-是正負符號
            stack: Vec::new(),
            in_coming: HashMap::new()
        };

        // priority.in_coming.insert('+', 3);
        // priority.in_coming.insert('-', 3);
        priority.in_coming.insert('*', 2);
        priority.in_coming.insert('/', 2);
        priority.in_coming.insert('%', 2);
        priority.in_coming.insert('+', 1);
        priority.in_coming.insert('-', 1);
        priority.in_coming.insert('(', 4);
        priority.in_coming.insert(')', 0);

        for c in expr.chars() {
            // 如果運算子有設定，傳回相對的優先權，否則優先權設定為-1
            let pv = match priority.in_coming.get(&c) {
                Some(v) => *v,
                None => -1,
            };

            match c {
                '+' | '-' => match priority.pos_neg {
                    true => priority.stack.push(OperPriority { ch: c, priority: pv + 2 }),  // 是正負符號，優先權+2
                    false => {
                        priority.stack.push(OperPriority { ch: c, priority: pv });
                        priority.pos_neg = true;    // 下一個遇到的+、-是正負符號
                    }
                },
                _ => {
                    priority.stack.push(OperPriority { ch: c, priority: pv });

                    if "*/%(".contains(c) {
                        priority.pos_neg = true;    // 下一個遇到的+、-是正負符號
                    } else {
                        priority.pos_neg = false;   // 下一個遇到的+、-不是正負符號
                    }
                }
            }
        }

        priority
    }

    pub fn show(&self) {
        let mut stack: Vec<OperPriority> = Vec::new();

        for in_coming in &self.stack {
            if in_coming.priority == -1 {
                print!("{}", in_coming.ch);     // 運算元直接輸出
            } else {
                loop {
                    match stack.pop() {
                        Some(v) => {
                            if  v.priority >= in_coming.priority {
                                if v.ch != '(' {
                                    print!("{}", v.ch);
                                }
                            } else {
                                stack.push(v);

                                match in_coming.ch {
                                    '(' => stack.push(OperPriority { ch: in_coming.ch, priority: 0 }),  // 放入堆疊前改變優先權
                                    _ => stack.push(in_coming.clone())
                                }

                                break;
                            }
                        },
                        None => {
                            match in_coming.ch {
                                '(' => stack.push(OperPriority { ch: in_coming.ch, priority: 0}),   // 放入堆疊前改變優先權
                                _ => stack.push(in_coming.clone())
                            }

                            break;
                        }
                    }
                }
            }
        }

        loop {
            match stack.pop() {
                Some(v) if v.ch != '(' && v.ch != ')' => print!("{}", v.ch),
                _ => break
            }
        }

        println!("");
    }
}