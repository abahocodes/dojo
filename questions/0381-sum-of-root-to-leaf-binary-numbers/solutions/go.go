package main

func sumRootToLeaf(root *TreeNode) int {
	type item struct {
		node   *TreeNode
		prefix int
	}
	total := 0
	var stack []item
	if root != nil {
		stack = append(stack, item{root, 0})
	}
	for len(stack) > 0 {
		top := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		value := top.prefix*2 + top.node.Val
		if top.node.Left == nil && top.node.Right == nil {
			total += value
			continue
		}
		if top.node.Left != nil {
			stack = append(stack, item{top.node.Left, value})
		}
		if top.node.Right != nil {
			stack = append(stack, item{top.node.Right, value})
		}
	}
	return total
}
