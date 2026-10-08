package main

import (
	"sort"
	"strings"
)

func removeSubfolders(folder []string) []string {
	sorted := append([]string(nil), folder...)
	sort.Strings(sorted)
	result := []string{}
	for _, path := range sorted {
		if len(result) == 0 || !strings.HasPrefix(path, result[len(result)-1]+"/") {
			result = append(result, path)
		}
	}
	return result
}
