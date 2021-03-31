package main

import "fmt"

func main() {
	fmt.Println(lengthOfLongestSubstring("bbbbb"))
}

func lengthOfLongestSubstring(s string) int {
	maxLen := 0
	length := 0
	m := make(map[string]int)
	for i := 0; i < len(s); i++ {
		if l, ok := m[string(s[i])]; ok && l >= i-length {
			length = i - l
		} else {
			length++
		}
		m[string(s[i])] = i
		maxLen = max(maxLen, length)
	}

	return maxLen
}

func max(i, j int) int {
	if i > j {
		return i
	} else {
		return j
	}
}
