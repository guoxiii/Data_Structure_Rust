use std::collections::HashMap;

fn calc_postfix(pf: &str) -> f64 {
    let mut values = HashMap::new();
    let mut stack: Vec<f64> = Vec::new();

    values.insert('0', 0.0);
    values.insert('A', 59.0);
    values.insert('B', 62.0);
    values.insert('C', 26.0);
    values.insert('D', 11.0);
    values.insert('E', 24.0);
    values.insert('F', 19.0);
    values.insert('G', 10.0);

    for c in pf.chars() {
        if "0ABCDEFG".contains(c) {
            stack.push(*values.get(&c).unwrap());
        } else {
            let a = stack.pop().unwrap_or(0.0);
            let b = stack.pop().unwrap_or(0.0);

            match c {
                '+' => stack.push(b + a),
                '-' => stack.push(b - a),
                '*' => stack.push(b * a),
                '/' => stack.push(b / a),
                '%' => stack.push(b % a),
                _ =>  {}
            }
        }
    }

    stack.pop().unwrap()
}

fn main() {
    // A*B+C = 3684
    println!("AB*C+ = {}", calc_postfix("AB*C+"));

    // A-B/C+D*E-F%G = 311.6154
    println!("ABC/-DE*+FG%- = {}", calc_postfix("ABC/-DE*+FG%-"));

    // A+B*C = 1671
    println!("ABC*+ = {}", calc_postfix("ABC*+"));
    // A*(B+C)*D = 57112
    println!("ABC+*D* = {}", calc_postfix("ABC+*D*"));

    // (-A)*(B+C)*(-D)，計算後答案得到-0，錯誤!!!!!(正確答案應該是57112)
    println!("A-BC+*D-* = {}", calc_postfix("A-BC+*D-*"));

    // (0-A)*(B+C)*(0-D) = 57112，中序轉換前先補一個0值即可
    println!("0A-BC+*0D-* = {}", calc_postfix("0A-BC+*0D-*"));
}
