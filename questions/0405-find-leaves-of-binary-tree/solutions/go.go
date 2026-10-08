package main

func findLeaves(root *TreeNode) [][]int {
	var result [][]int
	var height func(node *TreeNode) int
	height = func(node *TreeNode) int {
		if node == nil {
			return -1
		}
		h := max(height(node.Left), height(node.Right)) + 1
		if h == len(result) {
			result = append(result, []int{})
		}
		result[h] = append(result[h], node.Val)
		return h
	}
	height(root)
	return result
}
