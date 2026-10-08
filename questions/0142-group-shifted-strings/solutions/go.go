package main

func groupStrings(strings []string) [][]string {
	index := map[string]int{}
	groups := [][]string{}
	for _, s := range strings {
		key := make([]byte, len(s))
		for i := 0; i < len(s); i++ {
			key[i] = byte('a' + (int(s[i])-int(s[0])+26)%26)
		}
		k := string(key)
		if at, ok := index[k]; ok {
			groups[at] = append(groups[at], s)
		} else {
			index[k] = len(groups)
			groups = append(groups, []string{s})
		}
	}
	return groups
}
