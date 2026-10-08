package main

func inorderSuccessor(root *TreeNode, p int) int {
	answer := -1
	node := root
	for node != nil {
		if node.Val > p {
			answer = node.Val
			node = node.Left
		} else {
			node = node.Right
		}
	}
	return answer
}
