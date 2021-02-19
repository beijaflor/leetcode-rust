    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut flags: Vec<bool> = Vec::with_capacity(s.len());
        let chars: Vec<char> = s.chars().collect();
        s.chars().for_each(|v| if v == '(' || v == ')' { flags.push(false) } else { flags.push(true) });
        println!("{:?}", flags);

        let mut lp = 0;
        let mut rp = s.len();
        'outer: while lp < rp {
            println!("while: {} / {}", lp, rp);
            for index in lp..rp {
                lp = index;
                if chars[lp] == '(' {
                    println!("left: {}", lp);
                    for index in (lp..rp).rev() {
                        rp = index;
                        if chars[rp] == ')' {
                            println!("right: {}", rp);
                            flags[lp] = true;
                            flags[rp] = true;
                            lp += 1;
                            rp -= 1;
                            continue 'outer
                        }
                    }
                }
            }
            break
        }
        println!("{:?}", flags);

        let mut index = 0;
        s.chars().filter(|_| {
            index += 1;
            flags[index -1]
        }).collect()
    }




fn main() {
    let result = min_remove_to_make_valid(String::from("lee(t(c)o)de)"));
    println!("{}", result);
    assert!(
        vec![
            String::from("lee(t(c)o)de"),
            String::from("lee(t(co)de)"),
            String::from("lee(t(c)ode)"),
        ].iter().find(|v| **v == result).is_some()
    );
    println!("SUCCESS\n\n");

    let result = min_remove_to_make_valid(String::from("a)b(c)d"));
    println!("{}", result);
    assert!(
        vec![
            String::from("ab(c)d"),
        ].iter().find(|v| **v == result).is_some()
    );
    println!("SUCCESS\n\n");

    let result = min_remove_to_make_valid(String::from("))(("));
    println!("{}", result);
    assert!(
        vec![
            String::from(""),
        ].iter().find(|v| **v == result).is_some()
    );
    println!("SUCCESS\n\n");

    let result = min_remove_to_make_valid(String::from("(a(b(c)d)"));
    println!("{}", result);
    assert!(
        vec![
            String::from("a(b(c)d)"),
        ].iter().find(|v| **v == result).is_some()
    );
    println!("SUCCESS\n\n");
}
