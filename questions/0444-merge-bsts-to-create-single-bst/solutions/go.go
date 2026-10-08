package main

func canMerge(trees []*TreeNode) *TreeNode {
	roots := make(map[int]*TreeNode, len(trees))
	leafValues := make(map[int]bool)
	for _, t := range trees {
		roots[t.Val] = t
		if t.Left != nil {
			leafValues[t.Left.Val] = true
		}
		if t.Right != nil {
			leafValues[t.Right.Val] = true
		}
	}

	var root *TreeNode
	candidates := 0
	for _, t := range trees {
		if !leafValues[t.Val] {
			root = t
			candidates++
		}
	}
	if candidates != 1 {
		return nil
	}
	delete(roots, root.Val)

	// Iterative DFS carrying the open interval (lo, hi) each node must fit in.
	type frame struct {
		node   *TreeNode
		lo, hi int
	}
	stack := []frame{{root, 0, 1 << 31}}
	for len(stack) > 0 {
		f := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		node := f.node
		if node.Val <= f.lo || node.Val >= f.hi {
			return nil
		}
		if node.Left == nil && node.Right == nil {
			if sub, ok := roots[node.Val]; ok {
				node.Left, node.Right = sub.Left, sub.Right
				delete(roots, node.Val)
			}
		}
		if node.Left != nil {
			stack = append(stack, frame{node.Left, f.lo, node.Val})
		}
		if node.Right != nil {
			stack = append(stack, frame{node.Right, node.Val, f.hi})
		}
	}

	if len(roots) != 0 {
		return nil
	}
	return root
}
