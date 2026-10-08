package main

func minDepth(root *TreeNode) int {
	if root == nil {
		return 0
	}
	level := []*TreeNode{root}
	depth := 1
	for len(level) > 0 {
		var next []*TreeNode
		for _, node := range level {
			if node.Left == nil && node.Right == nil {
				return depth
			}
			if node.Left != nil {
				next = append(next, node.Left)
			}
			if node.Right != nil {
				next = append(next, node.Right)
			}
		}
		level = next
		depth++
	}
	return 0
}
