package main

func insertionSortList(head *ListNode) *ListNode {
	dummy := &ListNode{}
	var tail *ListNode
	cur := head
	for cur != nil {
		nxt := cur.Next
		if tail != nil && tail.Val <= cur.Val {
			tail.Next = cur
			cur.Next = nil
			tail = cur
		} else {
			p := dummy
			for p.Next != nil && p.Next.Val <= cur.Val {
				p = p.Next
			}
			cur.Next = p.Next
			p.Next = cur
			if cur.Next == nil {
				tail = cur
			}
		}
		cur = nxt
	}
	return dummy.Next
}
