package main

func isCompleteTree(root *TreeNode) bool {
	queue := []*TreeNode{root}
	seenGap := false
	for head := 0; head < len(queue); head++ {
		node := queue[head]
		if node == nil {
			seenGap = true
			continue
		}
		if seenGap {
			return false
		}
		queue = append(queue, node.Left, node.Right)
	}
	return true
}
