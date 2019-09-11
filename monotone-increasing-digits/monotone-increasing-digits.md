# two-sum

## 题目路径
https://leetcode-cn.com/problems/monotone-increasing-digits/

## 题型分类
循环

## 思路
一个数字比如99998，如果输出满足题意，那么输出必然是99998，99990 - 1，99900 - 1，99000 - 1，90000 - 1中的一个；
算法具体如下：
1.生成数列中的一个元素；
2.判断是否满足题意，若是，输出；若否，进入步骤1.

# 实现&用例
```
//生成数列下一个元素
fn isIncreasing(n: i32) -> bool {
    let mut num = n;
    let mut lastDigits = num % 10;
    while(num > 9) {
        num = num / 10;
        let mut newDigits = num % 10;
        if (lastDigits < newDigits) {
            return false;
        }
        lastDigits = newDigits;
    }
    return true;
}
impl Solution {
    pub fn monotone_increasing_digits(n: i32) -> i32 {
        // + 1是为了输出时一致
        let mut num = n + 1;
        let mut i = 1;
        while(!isIncreasing(num - 1)) {
            num = num - (num % (10 * i));
            i = i * 10;
        }
        return num - 1;
    }
}

```

```
mod monotone_increasing_digits;

#[cfg(test)]
mod tests {
    use crate::monotone_increasing_digits::monotone_increasing_digits;
    #[test]
    fn test_monotone_increasing_digits() {
        assert_eq!(9, monotone_increasing_digits(9));
        assert_eq!(9, monotone_increasing_digits(10));
        assert_eq!(99998, monotone_increasing_digits(89999));
        assert_eq!(1234, monotone_increasing_digits(1234));
    }
}

```

## 复杂度分析
空间复杂度O(1)
时间复杂度O(logn*logn)
两个循环都只依赖于数字位数

## 比较
贪心算法时间复杂度为O(logn)
但需要将数字转换为数组，空间复杂度增大为O(logn)

## 结果
>执行用时 :0 ms, 在所有 Rust 提交中击败了100.00%的用户
内存消耗 :2 MB, 在所有 Rust 提交中击败了100.00%的用户