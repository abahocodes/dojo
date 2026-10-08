package main

import (
	"sort"
	"strconv"
	"strings"
)

func largestNumber(nums []int) string {
	parts := make([]string, len(nums))
	for i, x := range nums {
		parts[i] = strconv.Itoa(x)
	}
	sort.Slice(parts, func(i, j int) bool {
		return parts[i]+parts[j] > parts[j]+parts[i]
	})
	if parts[0] == "0" {
		return "0"
	}
	return strings.Join(parts, "")
}
