package main

func isLeafNode(node *TreeNode) bool {
	return node.Left == nil && node.Right == nil
}

func boundaryOfBinaryTree(root *TreeNode) []int {
	result := []int{root.Val}
	if isLeafNode(root) {
		return result
	}

	node := root.Left
	for node != nil && !isLeafNode(node) {
		result = append(result, node.Val)
		if node.Left != nil {
			node = node.Left
		} else {
			node = node.Right
		}
	}

	stack := []*TreeNode{root}
	for len(stack) > 0 {
		cur := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		if isLeafNode(cur) {
			result = append(result, cur.Val)
			continue
		}
		if cur.Right != nil {
			stack = append(stack, cur.Right)
		}
		if cur.Left != nil {
			stack = append(stack, cur.Left)
		}
	}

	var right []int
	node = root.Right
	for node != nil && !isLeafNode(node) {
		right = append(right, node.Val)
		if node.Right != nil {
			node = node.Right
		} else {
			node = node.Left
		}
	}
	for i := len(right) - 1; i >= 0; i-- {
		result = append(result, right[i])
	}
	return result
}
