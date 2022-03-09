use std::collections::HashMap;

// 儲存到堆疊中的結構體
struct OperPriority {
    ch: char,       // 運算子
    priority: i32   // 優先權
}

pub struct InfixToPostfix {
    in_stack_priority: HashMap<char, i32>,   // 堆疊中的運算子優先權
    in_coming_priority: HashMap<char, i32>,  // 運算式中的運算子優先權
}

impl Default for InfixToPostfix {
    fn default() -> Self {
        let mut itp = Self {
            in_stack_priority: HashMap::new(),  // 設定要放入堆疊中時，運算子的優先權
            in_coming_priority: HashMap::new()  // 設定運算式中，運算子的優先權
        };

        itp.in_stack_priority.insert('*', 2);
        itp.in_stack_priority.insert('/', 2);
        itp.in_stack_priority.insert('%', 2);
        itp.in_stack_priority.insert('+', 1);  // 加
        itp.in_stack_priority.insert('-', 1);  // 減
        itp.in_stack_priority.insert('(', 0);
        itp.in_stack_priority.insert(')', 0);

        itp.in_coming_priority.insert('*', 2);
        itp.in_coming_priority.insert('/', 2);
        itp.in_coming_priority.insert('%', 2);
        itp.in_coming_priority.insert('+', 1);  // 加
        itp.in_coming_priority.insert('-', 1);  // 減
        itp.in_coming_priority.insert('(', 4);
        itp.in_coming_priority.insert(')', 0);

        itp
    }
}

impl InfixToPostfix {
    pub fn new() -> Self {
        Default::default()
    }
}

impl InfixToPostfix {
    pub fn to_postfix(&self, infix: &str) -> String {
        let mut stack: Vec<OperPriority> = Vec::new();
        let mut postfix_str = String::from("");

        for c in infix.chars() {    
            match self.in_coming_priority.get(&c) {
                Some(in_coming_priority) => {
                    loop {
                        match stack.pop() {
                            Some(op) => {
                                if op.priority >= *in_coming_priority {
                                    if op.ch != '(' {
                                        postfix_str.push(op.ch);
                                    }
                                } else {
                                    if op.ch != ')' {
                                        stack.push(op);
                                    }

                                    stack.push(OperPriority{ ch: c, priority: *self.in_stack_priority.get(&c).unwrap() });
                                    
                                    break;
                                }
                            },
                            None => {
                                if c != ')' {
                                    stack.push(OperPriority{ ch: c, priority: *self.in_stack_priority.get(&c).unwrap() });
                                }

                                break;
                            }
                        }
                    }
                },
                None => postfix_str.push(c)
            }            
        }

        loop {
            match stack.pop() {
                Some(v) => postfix_str.push(v.ch),
                None => break
            }
        }

        postfix_str
    }
}