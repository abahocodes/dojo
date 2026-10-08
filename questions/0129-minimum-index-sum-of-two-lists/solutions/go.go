package main

func findRestaurant(list1 []string, list2 []string) []string {
	index := make(map[string]int, len(list1))
	for i, s := range list1 {
		index[s] = i
	}
	best := -1
	result := []string{}
	for j, s := range list2 {
		i, ok := index[s]
		if !ok {
			continue
		}
		total := i + j
		if best == -1 || total < best {
			best = total
			result = []string{s}
		} else if total == best {
			result = append(result, s)
		}
	}
	return result
}
