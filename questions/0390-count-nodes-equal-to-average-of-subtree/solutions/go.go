package main

func averageOfSubtree(root *TreeNode) int {
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
	sums := map[*TreeNode]int{}
	sizes := map[*TreeNode]int{}
	count := 0
	for i := len(order) - 1; i >= 0; i-- {
		node := order[i]
		s, c := node.Val, 1
		for _, child := range []*TreeNode{node.Left, node.Right} {
			if child != nil {
				s += sums[child]
				c += sizes[child]
			}
		}
		sums[node] = s
		sizes[node] = c
		if s/c == node.Val {
			count++
		}
	}
	return count
}
