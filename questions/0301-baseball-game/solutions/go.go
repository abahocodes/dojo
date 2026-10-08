package main

import "strconv"

func calPoints(operations []string) int {
	record := []int{}
	for _, op := range operations {
		n := len(record)
		switch op {
		case "+":
			record = append(record, record[n-1]+record[n-2])
		case "D":
			record = append(record, 2*record[n-1])
		case "C":
			record = record[:n-1]
		default:
			v, _ := strconv.Atoi(op)
			record = append(record, v)
		}
	}
	total := 0
	for _, v := range record {
		total += v
	}
	return total
}
