package main

import (
	"testing"
)

func Benchmark_add2Nums(b *testing.B) {
	l1 := &ListNode{
		Val: 1,
		Next: &ListNode{
			Val: 2,
			Next: &ListNode{
				Val: 0,
				Next: &ListNode{
					Val: 1,
				},
			},
		},
	}
	l2 := &ListNode{
		Val: 3,
		Next: &ListNode{
			Val: 2,
			Next: &ListNode{
				Val: 0,
				Next: &ListNode{
					Val: 9,
					Next: &ListNode{
						Val: 5,
					},
				},
			},
		},
	}
	AddTwo(l1, l2)

}
