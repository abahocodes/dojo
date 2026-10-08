package main

func topKFrequent(nums []int, k int) []int {
	counts := make(map[int]int)
	for _, x := range nums {
		counts[x]++
	}

	// buckets[f] holds every value that occurs exactly f times
	// (filled in first-seen order, since Go's map iteration order is random)
	buckets := make([][]int, len(nums)+1)
	for _, value := range nums {
		if freq, ok := counts[value]; ok {
			buckets[freq] = append(buckets[freq], value)
			delete(counts, value)
		}
	}

	result := make([]int, 0, k)
	for freq := len(nums); freq > 0; freq-- {
		for _, value := range buckets[freq] {
			result = append(result, value)
			if len(result) == k {
				return result
			}
		}
	}
	return result
}
