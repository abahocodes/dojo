package main

func kthDistinct(arr []string, k int) string {
	count := make(map[string]int)
	for _, s := range arr {
		count[s]++
	}
	for _, s := range arr {
		if count[s] == 1 {
			k--
			if k == 0 {
				return s
			}
		}
	}
	return ""
}
