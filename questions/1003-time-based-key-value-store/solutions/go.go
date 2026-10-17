package main

import "sort"

type entry struct {
	timestamp int
	value     string
}

type TimeMap struct {
	// key -> writes; set timestamps only increase, so each list is sorted by time.
	history map[string][]entry
}

func Constructor() TimeMap {
	return TimeMap{history: map[string][]entry{}}
}

func (this *TimeMap) Set(key string, value string, timestamp int) {
	this.history[key] = append(this.history[key], entry{timestamp, value})
}

func (this *TimeMap) Get(key string, timestamp int) string {
	h := this.history[key]
	// First write after `timestamp`; the one before it is the answer.
	i := sort.Search(len(h), func(i int) bool { return h[i].timestamp > timestamp })
	if i == 0 {
		return ""
	}
	return h[i-1].value
}

// dojo calls this to run the tests; you don't need to change it.
func timeMap(ops []string, keys []string, values []string, timestamps []int) []string {
	store := Constructor()
	out := []string{}
	for i, op := range ops {
		if op == "set" {
			store.Set(keys[i], values[i], timestamps[i])
		} else {
			out = append(out, store.Get(keys[i], timestamps[i]))
		}
	}
	return out
}
