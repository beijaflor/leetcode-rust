// https://leetcode.com/submissions/detail/480090760/
// #[derive(Debug, PartialEq, Eq)]
// pub enum NestedInteger {
//   Int(i32),
//   List(Vec<NestedInteger>)
// }
struct NestedIterator {
    list: Vec<i32>,
    pos: usize,
}

fn flatten(nestedList: Vec<NestedInteger>) -> Vec<i32> {
    nestedList.into_iter().flat_map(|vec| {
        match vec {
            NestedInteger::Int(i) => vec![i],
            NestedInteger::List(vec) => flatten(vec),
        }
    }).collect::<Vec<i32>>()
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NestedIterator {

    fn new(nestedList: Vec<NestedInteger>) -> Self {
        NestedIterator {
            list: flatten(nestedList),
            pos: 0,
        }
    }
    
    fn next(&mut self) -> i32 {
        let result = self.list.get(self.pos).unwrap();
        self.pos += 1;
        *result
    }
    
    fn has_next(&self) -> bool {
        match self.list.get(self.pos) {
            None => false,
            Some(_) => true,
        }
    }
}

/**
 * Your NestedIterator object will be instantiated and called as such:
 * let obj = NestedIterator::new(nestedList);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */