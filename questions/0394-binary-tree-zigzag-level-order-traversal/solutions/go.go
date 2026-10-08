package main

func zigzagLevelOrder(root *TreeNode) [][]int {
	result := [][]int{}
	var level []*TreeNode
	if root != nil {
		level = append(level, root)
	}
	leftToRight := true
	for len(level) > 0 {
		n := len(level)
		values := make([]int, n)
		var next []*TreeNode
		for i, node := range level {
			if leftToRight {
				values[i] = node.Val
			} else {
				values[n-1-i] = node.Val
			}
			if node.Left != nil {
				next = append(next, node.Left)
			}
			if node.Right != nil {
				next = append(next, node.Right)
			}
		}
		result = append(result, values)
		level = next
		leftToRight = !leftToRight
	}
	return result
}
