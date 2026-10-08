package main

func sortedArrayToBst(nums []int) *TreeNode {
	var build func(lo, hi int) *TreeNode
	build = func(lo, hi int) *TreeNode {
		if lo > hi {
			return nil
		}
		mid := (lo + hi) / 2
		return &TreeNode{Val: nums[mid], Left: build(lo, mid-1), Right: build(mid+1, hi)}
	}
	return build(0, len(nums)-1)
}
