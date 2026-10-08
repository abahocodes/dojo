package main

func createBinaryTree(descriptions [][]int) *TreeNode {
	nodes := map[int]*TreeNode{}
	children := map[int]bool{}
	get := func(value int) *TreeNode {
		node, ok := nodes[value]
		if !ok {
			node = &TreeNode{Val: value}
			nodes[value] = node
		}
		return node
	}
	for _, d := range descriptions {
		parent, child := get(d[0]), get(d[1])
		if d[2] == 1 {
			parent.Left = child
		} else {
			parent.Right = child
		}
		children[d[1]] = true
	}
	for _, d := range descriptions {
		if !children[d[0]] {
			return nodes[d[0]]
		}
	}
	return nil
}
