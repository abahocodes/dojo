package main

func deleteAllDuplicates(head *ListNode) *ListNode {
	dummy := &ListNode{Next: head}
	prev, cur := dummy, head
	for cur != nil {
		if cur.Next != nil && cur.Next.Val == cur.Val {
			v := cur.Val
			for cur != nil && cur.Val == v {
				cur = cur.Next
			}
			prev.Next = cur
		} else {
			prev = cur
			cur = cur.Next
		}
	}
	return dummy.Next
}
