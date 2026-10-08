package main

func mergeNodes(head *ListNode) *ListNode {
	dummy := &ListNode{}
	tail := dummy
	total := 0
	for cur := head.Next; cur != nil; cur = cur.Next {
		if cur.Val == 0 {
			cur.Val = total
			tail.Next = cur
			tail = cur
			total = 0
		} else {
			total += cur.Val
		}
	}
	tail.Next = nil
	return dummy.Next
}
