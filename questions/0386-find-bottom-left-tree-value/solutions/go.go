package main

func findBottomLeftValue(root *TreeNode) int {
	queue := []*TreeNode{root}
	node := root
	for head := 0; head < len(queue); head++ {
		node = queue[head]
		if node.Right != nil {
			queue = append(queue, node.Right)
		}
		if node.Left != nil {
			queue = append(queue, node.Left)
		}
	}
	return node.Val
}
