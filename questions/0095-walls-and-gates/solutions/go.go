package main

const empty = 2147483647

func wallsAndGates(rooms [][]int) [][]int {
	rows, cols := len(rooms), len(rooms[0])
	dist := make([][]int, rows)
	var queue [][2]int
	for r := range dist {
		dist[r] = append([]int(nil), rooms[r]...)
		for c := 0; c < cols; c++ {
			if dist[r][c] == 0 {
				queue = append(queue, [2]int{r, c})
			}
		}
	}
	dirs := [4][2]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}
	for head := 0; head < len(queue); head++ {
		r, c := queue[head][0], queue[head][1]
		d := dist[r][c] + 1
		for _, dir := range dirs {
			nr, nc := r+dir[0], c+dir[1]
			if nr >= 0 && nr < rows && nc >= 0 && nc < cols && dist[nr][nc] == empty {
				dist[nr][nc] = d
				queue = append(queue, [2]int{nr, nc})
			}
		}
	}
	return dist
}
