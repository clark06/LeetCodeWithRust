#two-sum

##题目路径
https://leetcode-cn.com/problems/two-sum/

##题型分类
HashMap

##思路
遍历数组，看数组中是否存在另一个数，二者和为target（二次遍历）；
优化：创建一个集合，将已经遍历过的数放入集合，对于即将遍历的数，只需要看集合中是否有满足条件的数；
优化：寻找一种合适的数据结构来进行查找--HashMap

#实现&用例
```
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hashNums = HashMap::new();
        let mut index = 0;
        for num in nums {
            if let Some(indexLeft) = hashNums.get(&(target - num)) {
                return vec![*indexLeft as i32, index as i32];
            }
            hashNums.insert(num, index);
            index += 1;
        }
        return vec![-1, -1];
    }
}
```

```
mod two_sum;

#[cfg(test)]
mod tests {
    use crate::two_sum::two_sum;
    #[test]
    fn test_twosum() {
        assert_eq!(vec![0, 2], two_sum(vec![1, 2, 3, 4], 4));
        assert_eq!(vec![2, 3], two_sum(vec![1, 2, 3, 4], 7));
        assert_eq!(vec![0, 1], two_sum(vec![1, 2, 3, 4], 3));
        assert_eq!(vec![0, 1], two_sum(vec![1, 1, 3, 4], 2));
    }
}

```

##复杂度分析
HashMap只能保证在普通情况下查找的复杂度数O(1)，但在极端情况下，如果大量数据发生哈希碰撞，查找但复杂度会发生退化；


##结果
>执行用时 :0 ms, 在所有 Rust 提交中击败了100.00%的用户
内存消耗 :2.5 MB, 在所有 Rust 提交中击败了100.00%的用户