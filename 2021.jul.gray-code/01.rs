// https://leetcode.com/submissions/detail/515780652
impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let len = 1 << n;
        let mut result: Vec<i32> = Vec::with_capacity(len as usize);
        (0..len).for_each(|i| result.push(i ^ i >> 1));
        result
    }
}

/*
class Solution {
    public List<Integer> grayCode(int n) {
        List<Integer> result = new ArrayList<>();
        // there are 2 ^ n numbers in the Gray code sequence.
        int sequenceLength = 1 << n;
        for (int i = 0; i < sequenceLength; i++) {
            int num = i ^ i >> 1;
            result.add(num);
        }
        return result;
    }
}
*/