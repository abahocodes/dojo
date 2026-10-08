package main

func getAllElements(root1 *TreeNode, root2 *TreeNode) []int {
	inorder := func(root *TreeNode) []int {
		var values []int
		var stack []*TreeNode
		node := root
		for len(stack) > 0 || node != nil {
			for node != nil {
				stack = append(stack, node)
				node = node.Left
			}
			node = stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			values = append(values, node.Val)
			node = node.Right
		}
		return values
	}
	a, b := inorder(root1), inorder(root2)
	merged := make([]int, 0, len(a)+len(b))
	i, j := 0, 0
	for i < len(a) && j < len(b) {
		if a[i] <= b[j] {
			merged = append(merged, a[i])
			i++
		} else {
			merged = append(merged, b[j])
			j++
		}
	}
	merged = append(merged, a[i:]...)
	merged = append(merged, b[j:]...)
	return merged
}
