package main

import "container/list"

type lruEntry struct {
	key, value int
}

type LRUCache struct {
	capacity int
	order    *list.List // front is the most recently used, back the least
	nodes    map[int]*list.Element
}

func Constructor(capacity int) LRUCache {
	return LRUCache{capacity: capacity, order: list.New(), nodes: map[int]*list.Element{}}
}

func (this *LRUCache) Get(key int) int {
	e, ok := this.nodes[key]
	if !ok {
		return -1
	}
	this.order.MoveToFront(e)
	return e.Value.(*lruEntry).value
}

func (this *LRUCache) Put(key int, value int) {
	if e, ok := this.nodes[key]; ok {
		e.Value.(*lruEntry).value = value
		this.order.MoveToFront(e)
		return
	}
	this.nodes[key] = this.order.PushFront(&lruEntry{key, value})
	if len(this.nodes) > this.capacity {
		oldest := this.order.Back()
		this.order.Remove(oldest)
		delete(this.nodes, oldest.Value.(*lruEntry).key)
	}
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
