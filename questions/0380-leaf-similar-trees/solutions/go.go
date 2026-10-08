package main

func leaves(root *TreeNode) []int {
	var out []int
	var stack []*TreeNode
	if root != nil {
		stack = append(stack, root)
	}
	for len(stack) > 0 {
		node := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		if node.Left == nil && node.Right == nil {
			out = append(out, node.Val)
			continue
		}
		if node.Right != nil {
			stack = append(stack, node.Right)
		}
		if node.Left != nil {
			stack = append(stack, node.Left)
		}
	}
	return out
}

func leafSimilar(root1 *TreeNode, root2 *TreeNode) bool {
	a, b := leaves(root1), leaves(root2)
	if len(a) != len(b) {
		return false
	}
	for i := range a {
		if a[i] != b[i] {
			return false
		}
	}
	return true
}
