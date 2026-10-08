package main

func averageOfLevels(root *TreeNode) []float64 {
	averages := []float64{}
	level := []*TreeNode{root}
	for len(level) > 0 {
		total := 0
		var next []*TreeNode
		for _, node := range level {
			total += node.Val
			if node.Left != nil {
				next = append(next, node.Left)
			}
			if node.Right != nil {
				next = append(next, node.Right)
			}
		}
		averages = append(averages, float64(total)/float64(len(level)))
		level = next
	}
	return averages
}
