package main

func isBalanced(root *TreeNode) bool {
	if root == nil {
		return true
	}
	// reversed pre-order puts every child before its parent
	var order []*TreeNode
	stack := []*TreeNode{root}
	for len(stack) > 0 {
		node := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		order = append(order, node)
		if node.Left != nil {
			stack = append(stack, node.Left)
		}
		if node.Right != nil {
			stack = append(stack, node.Right)
		}
	}
	height := map[*TreeNode]int{} // nil -> 0
	for i := len(order) - 1; i >= 0; i-- {
		node := order[i]
		left, right := height[node.Left], height[node.Right]
		if left-right > 1 || right-left > 1 {
			return false
		}
		height[node] = 1 + max(left, right)
	}
	return true
}
