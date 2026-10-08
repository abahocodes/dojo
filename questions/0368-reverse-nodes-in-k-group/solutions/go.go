package main

func reverseKGroup(head *ListNode, k int) *ListNode {
	dummy := &ListNode{Val: 0, Next: head}
	groupPrev := dummy
	for {
		kth := groupPrev
		for i := 0; i < k; i++ {
			kth = kth.Next
			if kth == nil {
				return dummy.Next
			}
		}
		first := groupPrev.Next
		prev, cur := kth.Next, first
		for i := 0; i < k; i++ {
			next := cur.Next
			cur.Next = prev
			prev, cur = cur, next
		}
		groupPrev.Next = kth
		groupPrev = first
	}
}
