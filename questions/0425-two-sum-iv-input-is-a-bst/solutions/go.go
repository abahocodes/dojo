package main

func findTarget(root *TreeNode, k int) bool {
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

	i, j := 0, len(values)-1
	for i < j {
		s := values[i] + values[j]
		if s == k {
			return true
		}
		if s < k {
			i++
		} else {
			j--
		}
	}
	return false
}
