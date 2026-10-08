package main

func partitionList(head *ListNode, x int) *ListNode {
	small := &ListNode{}
	large := &ListNode{}
	s, l := small, large
	for cur := head; cur != nil; cur = cur.Next {
		if cur.Val < x {
			s.Next = cur
			s = cur
		} else {
			l.Next = cur
			l = cur
		}
	}
	l.Next = nil
	s.Next = large.Next
	return small.Next
}
