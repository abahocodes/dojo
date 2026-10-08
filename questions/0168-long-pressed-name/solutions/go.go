package main

func isLongPressedName(name string, typed string) bool {
	i := 0
	for j := 0; j < len(typed); j++ {
		c := typed[j]
		if i < len(name) && name[i] == c {
			i++
		} else if j == 0 || typed[j-1] != c {
			return false
		}
	}
	return i == len(name)
}
