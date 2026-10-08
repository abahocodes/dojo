package main

func isCousins(root *TreeNode, x int, y int) bool {
	level := []*TreeNode{root}
	for len(level) > 0 {
		var parentX, parentY *TreeNode
		var next []*TreeNode
		for _, node := range level {
			for _, child := range [2]*TreeNode{node.Left, node.Right} {
				if child == nil {
					continue
				}
				if child.Val == x {
					parentX = node
				} else if child.Val == y {
					parentY = node
				}
				next = append(next, child)
			}
		}
		if parentX != nil && parentY != nil {
			return parentX != parentY
		}
		if parentX != nil || parentY != nil {
			return false
		}
		level = next
	}
	return false
}
