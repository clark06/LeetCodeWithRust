# two-sum

## 题目路径
https://leetcode-cn.com/problems/invert-binary-tree/

## 题型分类
二叉树；遍历；递归；

## 思路
遍历二叉树所有节点，交换left和right；
递归借用函数调用栈来实现遍历，下文中但实现使用相同原理，显式得声明一个队列来完成遍历；

# 实现&用例
```
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut queue:VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        queue.push_back(root.clone());
        while (!queue.is_empty()) {
            if let Some(Some(node)) = queue.pop_back() {
                let left = node.borrow().left.clone();
                let right = node.borrow().right.clone();
                node.borrow_mut().left = right;
                node.borrow_mut().right = left;
                queue.push_back(node.borrow().left.clone());
                queue.push_back(node.borrow().right.clone());
            }
        }
        return root;
    }
}

```

```
#[cfg(test)]
mod tests {
	use crate::Solution;
	use crate::treenode::TreeNode;

	#[test]
	fn it_works() {
		let root = TreeNode::from_layer_vec(vec![
			Some(4),
			Some(2),
			Some(7),
			Some(1),
			Some(3),
			Some(6),
			Some(9),
		]);
		assert_eq!(Solution::invert_tree(root).unwrap().borrow().pre_order(),vec![4,7,2,9,6,3,1]);
	}
}

```

## 复杂度分析
空间复杂度O(n)
时间复杂度O(n)

## 困难点
pop_back的返回值是Option, 需要两次Some来获得对象,卡了好久；
可能有更优雅地实现，但目前没想出来；
另外，声明栈时嵌套了4层<>，也很不优雅

## 相似题
前序遍历二叉树
中序遍历二叉树
后序遍历二叉树

## 结果
>执行用时 :0 ms, 在所有 Rust 提交中击败了100.00%的用户
内存消耗 :2.1 MB, 在所有 Rust 提交中击败了100.00%的用户