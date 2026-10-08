package main

func distanceK(root *TreeNode, target int, k int) []int {
	// Turn the tree into an undirected graph keyed by value.
	adj := map[int][]int{root.Val: nil}
	stack := []*TreeNode{root}
	for len(stack) > 0 {
		node := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		for _, child := range []*TreeNode{node.Left, node.Right} {
			if child != nil {
				adj[node.Val] = append(adj[node.Val], child.Val)
				adj[child.Val] = append(adj[child.Val], node.Val)
				stack = append(stack, child)
			}
		}
	}
	frontier := []int{target}
	seen := map[int]bool{target: true}
	for step := 0; step < k && len(frontier) > 0; step++ {
		var next []int
		for _, v := range frontier {
			for _, w := range adj[v] {
				if !seen[w] {
					seen[w] = true
					next = append(next, w)
				}
			}
		}
		frontier = next
	}
	return frontier
}
