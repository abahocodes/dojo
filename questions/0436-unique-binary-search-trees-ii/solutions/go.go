package main

func generateTrees(n int) []*TreeNode {
	memo := map[[2]int][]*TreeNode{}
	var build func(lo, hi int) []*TreeNode
	build = func(lo, hi int) []*TreeNode {
		if lo > hi {
			return []*TreeNode{nil}
		}
		if trees, ok := memo[[2]int{lo, hi}]; ok {
			return trees
		}
		var trees []*TreeNode
		for v := lo; v <= hi; v++ {
			lefts := build(lo, v-1)
			rights := build(v+1, hi)
			for _, left := range lefts {
				for _, right := range rights {
					trees = append(trees, &TreeNode{Val: v, Left: left, Right: right})
				}
			}
		}
		memo[[2]int{lo, hi}] = trees
		return trees
	}
	return build(1, n)
}
