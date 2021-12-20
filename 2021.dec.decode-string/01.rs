// https://leetcode.com/submissions/detail/603771170/
#[derive(Debug)]
enum Ast {
    C(String),
    N(i32),
    OPEN,
    CLOSE,
}

fn tokenize(s: String) -> Vec<Ast> {
    let chars = s.chars().collect::<Vec<char>>();
    let mut tokens = Vec::<Ast>::new();
    let mut p = 0;
    while p < chars.len() {
        // println!("{}", p);
        if chars[p] == '[' {
            tokens.push(Ast::OPEN);
            p += 1;
        } else if chars[p] == ']' {
            tokens.push(Ast::CLOSE);
            p += 1;
        } else if '0' <= chars[p] && chars[p] <= '9' {
            let mut nums = Vec::<char>::new();
            while p < chars.len() && '0' <= chars[p] && chars[p] <= '9' {
                nums.push(chars[p]);
                p += 1;
            }
            tokens.push(Ast::N(nums.into_iter().collect::<String>().parse::<i32>().unwrap()));
        } else {
            let mut str = Vec::<char>::new();
            while p < chars.len() && !('0' <= chars[p] && chars[p] <= '9') && chars[p] != '[' && chars[p] != ']' {
                str.push(chars[p]);
                p += 1;
            }
            tokens.push(Ast::C(str.into_iter().collect::<String>()));
        }
    }
    tokens
}

fn decode(ast: &Vec<Ast>, p: &mut usize) -> String {
    let mut result = Vec::<String>::new();
    while *p < ast.len() {
        match &ast[*p] {
            Ast::C(str) => {
                result.push(str.clone());
                *p += 1;
            },
            Ast::N(num) => {
                *p += 2;
                let str = decode(ast, p);
                (0..*num).for_each(|_| {
                    result.push(str.clone());
                });
            },
            Ast::OPEN => {
                *p += 1;
            },
            Ast::CLOSE => {
                *p += 1;
                break
            },
        }
    }
//     let start = chars.iter().position(|c| {
        
//     });
    result.into_iter().collect()
}

impl Solution {
    pub fn decode_string(s: String) -> String {
        let ast = tokenize(s);
        // println!("{:?}", ast);
        decode(&ast, &mut 0)
    }
}