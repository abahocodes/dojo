package main

func sumNumbers(root *TreeNode) int {
	type item struct {
		node   *TreeNode
		prefix int
	}
	total := 0
	stack := []item{{root, 0}}
	for len(stack) > 0 {
		it := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		value := it.prefix*10 + it.node.Val
		if it.node.Left == nil && it.node.Right == nil {
			total += value
			continue
		}
		if it.node.Left != nil {
			stack = append(stack, item{it.node.Left, value})
		}
		if it.node.Right != nil {
			stack = append(stack, item{it.node.Right, value})
		}
	}
	return total
}
