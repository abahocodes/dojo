package main

func largestValues(root *TreeNode) []int {
	result := []int{}
	var level []*TreeNode
	if root != nil {
		level = append(level, root)
	}
	for len(level) > 0 {
		best := level[0].Val
		var next []*TreeNode
		for _, node := range level {
			if node.Val > best {
				best = node.Val
			}
			if node.Left != nil {
				next = append(next, node.Left)
			}
			if node.Right != nil {
				next = append(next, node.Right)
			}
		}
		result = append(result, best)
		level = next
	}
	return result
}
