package main

func deleteNodes(head *ListNode, m int, n int) *ListNode {
	cur := head
	for cur != nil {
		for i := 0; i < m-1; i++ {
			if cur.Next == nil {
				return head
			}
			cur = cur.Next
		}
		skip := cur.Next
		for i := 0; i < n && skip != nil; i++ {
			skip = skip.Next
		}
		cur.Next = skip
		cur = skip
	}
	return head
}
