pub struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut last: i32 = nums[0] - 1;
        let mut flag: bool = false;

        // retains elements in place as specified by predicate function
        nums.retain(|&x| {
            // record whether current and last are the same (assuming collection is sorted)
            flag = x == last;
            // update last
            last = x;
            //keep only if the value is differentn from the last
            !flag
        });

        nums.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0026(){
        let mut v1 = vec![1,1,2];
        let mut v2 = vec![0,0,1,1,1,2,2,3,3,4];
        assert_eq!(2, Solution::remove_duplicates(&mut v1));
        assert_eq!(&[1,2], &v1[..]);
        assert_eq!(5, Solution::remove_duplicates(&mut v2));
        assert_eq!(&[0,1,2,3,4], &v2[..]);
    }
}
