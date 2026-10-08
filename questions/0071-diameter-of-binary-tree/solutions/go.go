package main

func diameterOfBinaryTree(root *TreeNode) int {
	if root == nil {
		return 0
	}
	// visit nodes so that children come before their parent (reversed pre-order)
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
	height := map[*TreeNode]int{} // node -> number of nodes on its longest downward path (nil -> 0)
	best := 0
	for i := len(order) - 1; i >= 0; i-- {
		node := order[i]
		left, right := height[node.Left], height[node.Right]
		best = max(best, left+right)
		height[node] = 1 + max(left, right)
	}
	return best
}
