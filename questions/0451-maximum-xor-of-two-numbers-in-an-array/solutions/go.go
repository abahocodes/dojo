package main

const bits = 31 // every value is below 2^31

func findMaximumXor(nums []int) int {
	// Binary trie in a flat slice: child[2*node+bit] is the child index, 0 = none.
	child := make([]int32, 2*(len(nums)*bits+1))
	size := int32(1)
	best := 0
	for _, x := range nums {
		node := int32(0)
		for b := bits - 1; b >= 0; b-- {
			slot := 2*node + int32((x>>b)&1)
			if child[slot] == 0 {
				child[slot] = size
				size++
			}
			node = child[slot]
		}
		// Walk toward the opposite bit wherever possible.
		node = 0
		cur := 0
		for b := bits - 1; b >= 0; b-- {
			bit := int32((x >> b) & 1)
			if want := child[2*node+(bit^1)]; want != 0 {
				cur |= 1 << b
				node = want
			} else {
				node = child[2*node+bit]
			}
		}
		if cur > best {
			best = cur
		}
	}
	return best
}
