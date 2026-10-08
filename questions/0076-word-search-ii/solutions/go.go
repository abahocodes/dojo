package main

type trieNode struct {
	children [26]*trieNode
	count    int // number of non-nil children
	word     string
	hasWord  bool
}

func findWords(board [][]string, words []string) []string {
	root := &trieNode{}
	for _, word := range words {
		node := root
		for i := 0; i < len(word); i++ {
			k := word[i] - 'a'
			if node.children[k] == nil {
				node.children[k] = &trieNode{}
				node.count++
			}
			node = node.children[k]
		}
		node.word = word
		node.hasWord = true
	}

	rows, cols := len(board), len(board[0])
	grid := make([][]byte, rows)
	for r := range grid {
		grid[r] = make([]byte, cols)
		for c := range grid[r] {
			grid[r][c] = board[r][c][0]
		}
	}
	found := []string{}
	dirs := [4][2]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}

	var dfs func(r, c int, parent *trieNode)
	dfs = func(r, c int, parent *trieNode) {
		ch := grid[r][c]
		k := ch - 'a'
		node := parent.children[k]
		if node == nil {
			return
		}
		if node.hasWord {
			found = append(found, node.word)
			node.hasWord = false
		}
		grid[r][c] = '#'
		for _, d := range dirs {
			nr, nc := r+d[0], c+d[1]
			if nr >= 0 && nr < rows && nc >= 0 && nc < cols && grid[nr][nc] != '#' {
				dfs(nr, nc, node)
			}
		}
		grid[r][c] = ch
		// prune branches with nothing left to find
		if node.count == 0 && !node.hasWord {
			parent.children[k] = nil
			parent.count--
		}
	}

	for r := 0; r < rows; r++ {
		for c := 0; c < cols; c++ {
			dfs(r, c, root)
		}
	}
	return found
}
