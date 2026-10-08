package main

func maxAncestorDiff(root *TreeNode) int {
	type frame struct {
		node   *TreeNode
		lo, hi int
	}
	best := 0
	stack := []frame{{root, root.Val, root.Val}}
	for len(stack) > 0 {
		f := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		lo := min(f.lo, f.node.Val)
		hi := max(f.hi, f.node.Val)
		best = max(best, hi-lo)
		if f.node.Left != nil {
			stack = append(stack, frame{f.node.Left, lo, hi})
		}
		if f.node.Right != nil {
			stack = append(stack, frame{f.node.Right, lo, hi})
		}
	}
	return best
}
