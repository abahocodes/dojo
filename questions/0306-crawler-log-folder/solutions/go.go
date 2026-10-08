package main

func minOperationsToMain(logs []string) int {
	depth := 0
	for _, log := range logs {
		if log == "../" {
			if depth > 0 {
				depth--
			}
		} else if log != "./" {
			depth++
		}
	}
	return depth
}
