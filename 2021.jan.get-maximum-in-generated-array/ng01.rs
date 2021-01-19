fn find_num(n: i32) -> i32 {
    if n < 2 { return 1 }
    let odd = n % 2;
    let modd = n / 2;
    let find;
    
    if odd == 1 {
        find = find_num(modd) + find_num(modd + 1);
    } else {
        find = find_num(modd);
    }
    println!("num: {} => odd: {}, mod: {}, find: {}", n, odd, modd, find);
    find
}

    pub fn get_maximum_generated(n: i32) -> i32 {
        if n == 0 { return 0 }
        let odd = n % 2;
        if odd == 1 {
            println!("id odd");
            return find_num(n);
        } else {
            println!("id even");
            return find_num(n - 1);
        }
    }

fn main() {
    println!("{}", find_num(8));
    println!("--------\n");


    assert_eq!(
        3,
        get_maximum_generated(7)
    );
    println!("SUCCESS\n\n");

    assert_eq!(
        1,
        get_maximum_generated(2)
    );
    println!("SUCCESS\n\n");

    assert_eq!(
        2,
        get_maximum_generated(3)
    );
    println!("SUCCESS\n\n");

    assert_eq!(
        2,
        get_maximum_generated(4)
    );
    println!("SUCCESS\n\n");

    assert_eq!(
        3,
        get_maximum_generated(6)
    );
    println!("SUCCESS\n\n");

    assert_eq!(
        5,
        get_maximum_generated(15)
    );
    println!("SUCCESS\n\n");
}
