package main

func deepestLeavesSum(root *TreeNode) int {
	level := []*TreeNode{root}
	sum := 0
	for len(level) > 0 {
		sum = 0
		var next []*TreeNode
		for _, node := range level {
			sum += node.Val
			if node.Left != nil {
				next = append(next, node.Left)
			}
			if node.Right != nil {
				next = append(next, node.Right)
			}
		}
		level = next
	}
	return sum
}
