package main

func widthOfBinaryTree(root *TreeNode) int {
	type item struct {
		node *TreeNode
		pos  int
	}
	best := 0
	level := []item{{root, 0}}
	for len(level) > 0 {
		base := level[0].pos
		if w := level[len(level)-1].pos - base + 1; w > best {
			best = w
		}
		var next []item
		for _, it := range level {
			pos := it.pos - base
			if it.node.Left != nil {
				next = append(next, item{it.node.Left, 2 * pos})
			}
			if it.node.Right != nil {
				next = append(next, item{it.node.Right, 2*pos + 1})
			}
		}
		level = next
	}
	return best
}
