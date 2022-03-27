use std::collections::HashMap;

fn main() {
    println!("{}", fib(1))
}

pub fn fib(n: i32) -> i64 {
    if n <= 1 {
        return n as i64
    }
    let mut m: HashMap<i32, i64> = HashMap::new();
    m.insert(0, 0 as i64);
    m.insert(1, 1 as i64);
    fibr(n, &mut m)
}


pub fn fibr(n: i32, m: &mut HashMap<i32, i64>) -> i64 {
    if !m.contains_key(&(n - 1)) {
        let temp = fibr(n - 1, m);
        m.insert(n - 1, temp);
    }
    if !m.contains_key(&(n - 2)) {
        let temp = fibr(n - 2, m);
        m.insert(n - 1, temp);
    }
    *m.get(&(n - 1)).unwrap() + *m.get(&(n - 2)).unwrap()
}
