package main

func recoverFromPreorder(traversal string) *TreeNode {
	var stack []*TreeNode
	n, i := len(traversal), 0
	for i < n {
		depth := 0
		for traversal[i] == '-' {
			depth++
			i++
		}
		value := 0
		for i < n && traversal[i] != '-' {
			value = value*10 + int(traversal[i]-'0')
			i++
		}
		node := &TreeNode{Val: value}
		stack = stack[:depth]
		if depth > 0 {
			parent := stack[depth-1]
			if parent.Left == nil {
				parent.Left = node
			} else {
				parent.Right = node
			}
		}
		stack = append(stack, node)
	}
	return stack[0]
}
