package main

import "fmt"

func main() {
	a := []int{1, 2, 3, 4, 5, 6, 7}
	r1 := twosum(a, 8)
	r2 := twosum(a, 7)
	fmt.Println(r1)
	fmt.Println(r2)
}

func twosum(arr []int, target int) []int {
	m := make(map[int]int)
	for k, v := range arr {
		if idx, ok := m[target-v]; ok {
			return []int{idx, k}
		}
		m[v] = k
	}
	return nil
}
