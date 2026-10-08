package main

func removeZeroSumSublists(head *ListNode) *ListNode {
	dummy := &ListNode{Val: 0, Next: head}
	last := make(map[int]*ListNode)
	total := 0
	for node := dummy; node != nil; node = node.Next {
		total += node.Val
		last[total] = node
	}
	total = 0
	for node := dummy; node != nil; node = node.Next {
		total += node.Val
		node.Next = last[total].Next
	}
	return dummy.Next
}
