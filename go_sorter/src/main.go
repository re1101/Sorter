package main

import (
	"fmt"
	"math/rand"
	"time"
)

func main() {
	var opc int
	var arr []int

	for {
		fmt.Printf("\x1b[2J")
		fmt.Println("SORTER")
		fmt.Println("Add array manualy or automatically?")
		fmt.Println("(1): Manually; (2): Auto; (0): Exit")
		fmt.Scan(&opc)

		if opc == 1 {
			var aux1 uint
			fmt.Println("Enter the number of elements in the array:")
			fmt.Scan(&aux1)
			fmt.Println("Enter the elements in the array:")
			for i := 0; i < int(aux1); i++ {
				var aux2 int
				fmt.Scan(&aux2)
				arr = append(arr, aux2)
			}
		} else if opc == 2 {
			var aux uint
			fmt.Println("Enter the number of elements in the array:")
			fmt.Scan(&aux)
			arr = randArray(aux)
		} else if opc == 0 {
			return
		} else {
			fmt.Println("Invalid option. Please try again.")
		}

		if len(arr) <= 10 {
			fmt.Println("\nArray:", arr)
		} else {
			fmt.Println("\nArray (first 10 elements):", arr[:10])
			fmt.Println("...")
			fmt.Println("Array (last 10 elements):", arr[len(arr)-10:])
		}

		fmt.Println("\n Start Time: ", time.Now())
		for s := 0; s < 5; s++ {
			go displaySort(s, arr)
		}

		for {
			fmt.Println("\nWould you like to try another array? (Y/N)")
			var aux string
			fmt.Scan(&aux)
			if aux == "Y" {
				break
			} else if aux == "N" {
				return
			} else {
				fmt.Println("Invalid option. Please try again.")
			}
		}
	}
}

func displaySort(s int, arr []int) {
	var sort string
	start := time.Now()
	switch s {
	case 0:
		arr = bubble(arr)
		sort = "Bubble Sort"
	case 1:
		arr = selection(arr)
		sort = "Selection Sort"
	case 2:
		arr = insertion(arr)
		sort = "Insertion Sort"
	case 3:
		arr = merge(arr)
		sort = "Merge Sort"
	case 4:
		arr = quick(arr)
		sort = "Quick Sort"
	default:
		fmt.Println("Invalid sort option.")
	}
	if len(arr) <= 10 {
		fmt.Println("\n", sort, ": Time elapsed: ", time.Since(start), "; Sorted Array: ", arr)
	} else {
		fmt.Println("\n", sort, ": Time elapsed: ", time.Since(start))
	}
}

func bubble(arr []int) []int {
	res := arr

	n := len(res)
	for i := 0; i < n; i++ {
		for j := 0; j < (n - i - 1); j++ {
			if res[j] > res[j+1] {
				aux := res[j]
				res[j] = res[j+1]
				res[j+1] = aux
			}
		}
	}
	return res
}

func selection(arr []int) []int {
	res := arr

	n := len(res)
	for i := 0; i < n; i++ {
		min_idx := i
		for j := i + 1; j < n; j++ {
			if res[j] < res[min_idx] {
				min_idx = j
			}
		}
		aux := res[i]
		res[i] = res[min_idx]
		res[min_idx] = aux
	}
	return res
}

func insertion(arr []int) []int {
	res := arr

	n := len(res)
	for i := 1; i < n; i++ {
		key := res[i]
		j := i - 1
		neg := false
		for key < res[j] {
			res[j+1] = res[j]
			if j == 0 {
				neg = true
				break
			}
			j -= 1
		}
		if neg {
			res[j] = key
		} else {
			res[j+1] = key
		}
	}
	return res
}

func merge(arr []int) []int {
	var res []int = append([]int(nil), arr...)

	var n int = len(res)
	if n > 1 {
		var mid int = n / 2

		var l []int
		l = append([]int(nil), arr[:mid]...)
		var r []int
		r = append([]int(nil), arr[mid:]...)

		l = merge(l)
		r = merge(r)

		i, j, k := 0, 0, 0
		for i < len(l) && j < len(r) {
			if l[i] < r[j] {
				res[k] = l[i]
				i += 1
			} else {
				res[k] = r[j]
				j += 1
			}
			k += 1
		}

		for i < len(l) {
			res[k] = l[i]
			i += 1
			k += 1
		}

		for j < len(r) {
			res[k] = r[j]
			j += 1
			k += 1
		}
	}
	return res
}

func quick(arr []int) []int {
	var res []int
	n := len(arr)
	if n <= 1 {
		res = append([]int(nil), arr...)
	} else {
		var middP int = n / 2
		var pivot = arr[middP]
		left := []int(nil)
		middle := []int(nil)
		right := []int(nil)
		for i := 0; i < n; i++ {
			x := arr[i]
			if x < pivot {
				left = append(left, x)
			} else if x == pivot {
				middle = append(middle, x)
			} else if x > pivot {
				right = append(right, x)
			}
		}
		y := append(quick(left), middle...)
		res = append(y, quick(right)...)
	}
	return res
}

func randArray(nums uint) []int {
	var arr []int

	for i := 0; i < int(nums); i++ {
		arr = append(arr, rand.Intn(600)-100)
	}

	return arr
}
