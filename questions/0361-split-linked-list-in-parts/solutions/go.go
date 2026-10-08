package main

func splitListToParts(head *ListNode, k int) []*ListNode {
	n := 0
	for p := head; p != nil; p = p.Next {
		n++
	}
	base, extra := n/k, n%k
	parts := make([]*ListNode, k)
	cur := head
	for i := 0; i < k; i++ {
		size := base
		if i < extra {
			size++
		}
		parts[i] = cur
		for j := 0; j < size-1; j++ {
			cur = cur.Next
		}
		if size > 0 {
			nxt := cur.Next
			cur.Next = nil
			cur = nxt
		}
	}
	return parts
}
