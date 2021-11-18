struct CombinationIterator {
  chars: Vec<char>,
  indexes: Vec<usize>,
  len: usize,
  has_next: bool,
}


/** 
* `&self` means the method takes an immutable reference.
* If you need a mutable reference, change it to `&mut self` instead.
*/
impl CombinationIterator {

  fn new(characters: String, combinationLength: i32) -> Self {
      CombinationIterator {
          chars: characters.chars().collect(),
          indexes: vec![0; combinationLength as usize],
          len: combinationLength as usize,
          has_next: true,
      }
  }
  
  fn next(&mut self) -> String {
      let result = self.indexes.iter().map(|index| self.chars[*index]).collect();
      let mut carry = true;
      (0..self.len).rev().for_each(|index| {
          if carry {
              self.indexes[index] += 1;
              if self.indexes[index] == self.chars.len() {
                  carry = true;
                  self.indexes[index] = 0;
              } else {
                  carry = false;
              }
          }
      });
      if carry { self.has_next = false }
      result
  }
  
  fn has_next(&self) -> bool {
      self.has_next
  }
}

/**
* Your CombinationIterator object will be instantiated and called as such:
* let obj = CombinationIterator::new(characters, combinationLength);
* let ret_1: String = obj.next();
* let ret_2: bool = obj.has_next();
*/