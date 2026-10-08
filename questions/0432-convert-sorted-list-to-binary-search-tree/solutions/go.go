package main

func sortedListToBst(head *ListNode) *TreeNode {
	n := 0
	for node := head; node != nil; node = node.Next {
		n++
	}

	cur := head
	var build func(lo, hi int) *TreeNode
	build = func(lo, hi int) *TreeNode {
		if lo > hi {
			return nil
		}
		mid := (lo + hi + 1) / 2
		left := build(lo, mid-1)
		root := &TreeNode{Val: cur.Val, Left: left}
		cur = cur.Next
		root.Right = build(mid+1, hi)
		return root
	}
	return build(0, n-1)
}
