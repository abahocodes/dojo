package main

// deepest returns the height of the subtree and the root of the answer inside it.
func deepest(node *TreeNode) (int, *TreeNode) {
	if node == nil {
		return 0, nil
	}
	lh, la := deepest(node.Left)
	rh, ra := deepest(node.Right)
	if lh > rh {
		return lh + 1, la
	}
	if rh > lh {
		return rh + 1, ra
	}
	return lh + 1, node
}

func subtreeWithAllDeepest(root *TreeNode) *TreeNode {
	_, ans := deepest(root)
	return ans
}
