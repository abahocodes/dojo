package main

func canFinish(numCourses int, prerequisites [][]int) bool {
	unlocks := make([][]int, numCourses)
	indegree := make([]int, numCourses)
	for _, p := range prerequisites {
		course, before := p[0], p[1]
		unlocks[before] = append(unlocks[before], course)
		indegree[course]++
	}

	var ready []int
	for i := 0; i < numCourses; i++ {
		if indegree[i] == 0 {
			ready = append(ready, i)
		}
	}
	taken := 0
	for taken < len(ready) {
		current := ready[taken]
		taken++
		for _, nxt := range unlocks[current] {
			indegree[nxt]--
			if indegree[nxt] == 0 {
				ready = append(ready, nxt)
			}
		}
	}
	return taken == numCourses
}
