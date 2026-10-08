package main

func findTilt(root *TreeNode) int {
	if root == nil {
		return 0
	}
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
	subtreeSum := map[*TreeNode]int{} // nil children read as 0
	tilt := 0
	for i := len(order) - 1; i >= 0; i-- {
		node := order[i]
		left, right := subtreeSum[node.Left], subtreeSum[node.Right]
		if left > right {
			tilt += left - right
		} else {
			tilt += right - left
		}
		subtreeSum[node] = node.Val + left + right
	}
	return tilt
}
