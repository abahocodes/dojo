package main

func rightSideView(root *TreeNode) []int {
	if root == nil {
		return nil
	}
	var view []int
	level := []*TreeNode{root}
	for len(level) > 0 {
		view = append(view, level[len(level)-1].Val)
		var next []*TreeNode
		for _, node := range level {
			if node.Left != nil {
				next = append(next, node.Left)
			}
			if node.Right != nil {
				next = append(next, node.Right)
			}
		}
		level = next
	}
	return view
}
