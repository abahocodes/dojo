package main

func numComponents(head *ListNode, nums []int) int {
	wanted := make(map[int]bool, len(nums))
	for _, x := range nums {
		wanted[x] = true
	}
	count := 0
	for cur := head; cur != nil; cur = cur.Next {
		if wanted[cur.Val] && (cur.Next == nil || !wanted[cur.Next.Val]) {
			count++
		}
	}
	return count
}
