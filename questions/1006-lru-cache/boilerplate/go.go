// 1006. LRU Cache (medium)
// /show to reread the problem · /test runs visible tests · /submit when done
package main

type LRUCache struct {
}

func Constructor(capacity int) LRUCache {
	return LRUCache{}
}

func (this *LRUCache) Get(key int) int {
	return 0
}

func (this *LRUCache) Put(key int, value int) {

}

// dojo calls this to run the tests; you don't need to change it.
func lruCache(capacity int, ops []string, args [][]int) []int {
	cache := Constructor(capacity)
	out := []int{}
	for i, op := range ops {
		if op == "put" {
			cache.Put(args[i][0], args[i][1])
		} else {
			out = append(out, cache.Get(args[i][0]))
		}
	}
	return out
}
