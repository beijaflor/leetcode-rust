    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut open_count = 0;
        s.chars().filter(|v| {
            // println!("{}", open_count);
            if *v == ')' {
                if open_count != 0 {
                    open_count -= 1;
                    true
                } else {
                    false
                }
            } else if *v == '(' {
                open_count += 1;
                true
            } else {
                true
            }
        }).collect()
    }




fn main() {
    let mut result = min_remove_to_make_valid(String::from("lee(t(c)o)de)"));
    println!("{}", result);
    assert!(
        vec![
            String::from("lee(t(c)o)de"),
            String::from("lee(t(co)de)"),
            String::from("lee(t(c)ode)"),
        ].iter().find(|v| **v == result).is_some()
    );
    println!("SUCCESS\n\n");

    let mut result = min_remove_to_make_valid(String::from("a)b(c)d"));
    println!("{}", result);
    assert!(
        vec![
            String::from("ab(c)d"),
        ].iter().find(|v| **v == result).is_some()
    );
    println!("SUCCESS\n\n");

    let mut result = min_remove_to_make_valid(String::from("))(("));
    println!("{}", result);
    assert!(
        vec![
            String::from(""),
        ].iter().find(|v| **v == result).is_some()
    );
    println!("SUCCESS\n\n");

    let mut result = min_remove_to_make_valid(String::from("(a(b(c)d)"));
    println!("{}", result);
    assert!(
        vec![
            String::from("a(b(c)d)"),
        ].iter().find(|v| **v == result).is_some()
    );
    println!("SUCCESS\n\n");
}
