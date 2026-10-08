package main

func lowestCommonAncestor(root *TreeNode, p int, q int) int {
	parent := map[int]*TreeNode{root.Val: nil}
	stack := []*TreeNode{root}
	for len(stack) > 0 {
		_, hasP := parent[p]
		_, hasQ := parent[q]
		if hasP && hasQ {
			break
		}
		node := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		for _, child := range []*TreeNode{node.Left, node.Right} {
			if child != nil {
				parent[child.Val] = node
				stack = append(stack, child)
			}
		}
	}
	ancestors := map[int]bool{p: true}
	for up := parent[p]; up != nil; up = parent[up.Val] {
		ancestors[up.Val] = true
	}
	v := q
	for !ancestors[v] {
		v = parent[v].Val
	}
	return v
}
