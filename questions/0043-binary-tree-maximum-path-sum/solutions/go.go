package main

func maxPathSum(root *TreeNode) int {
	order := []*TreeNode{}
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

	gain := map[*TreeNode]int{}
	best := root.Val
	for k := len(order) - 1; k >= 0; k-- {
		node := order[k]
		left := max(gain[node.Left], 0)
		right := max(gain[node.Right], 0)
		best = max(best, node.Val+left+right)
		gain[node] = node.Val + max(left, right)
	}
	return best
}
