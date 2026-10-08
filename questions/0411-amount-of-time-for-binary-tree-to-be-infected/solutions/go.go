package main

func amountOfTime(root *TreeNode, start int) int {
	parent := map[*TreeNode]*TreeNode{root: nil}
	var source *TreeNode
	stack := []*TreeNode{root}
	for len(stack) > 0 {
		node := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		if node.Val == start {
			source = node
		}
		for _, child := range []*TreeNode{node.Left, node.Right} {
			if child != nil {
				parent[child] = node
				stack = append(stack, child)
			}
		}
	}

	seen := map[*TreeNode]bool{source: true}
	frontier := []*TreeNode{source}
	minutes := -1
	for len(frontier) > 0 {
		minutes++
		var next []*TreeNode
		for _, node := range frontier {
			for _, neighbor := range []*TreeNode{node.Left, node.Right, parent[node]} {
				if neighbor != nil && !seen[neighbor] {
					seen[neighbor] = true
					next = append(next, neighbor)
				}
			}
		}
		frontier = next
	}
	return minutes
}
