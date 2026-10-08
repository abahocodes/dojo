package main

func maxDepth(root *TreeNode) int {
	if root == nil {
		return 0
	}
	depth := 0
	level := []*TreeNode{root}
	for len(level) > 0 {
		depth++
		var nxt []*TreeNode
		for _, node := range level {
			if node.Left != nil {
				nxt = append(nxt, node.Left)
			}
			if node.Right != nil {
				nxt = append(nxt, node.Right)
			}
		}
		level = nxt
	}
	return depth
}
