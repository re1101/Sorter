package main

import (
	"fmt"
	"math/rand"
)

func main() {
	for {
		var opc int
		var arr []int

		fmt.Printf("\x1b[2J")
		fmt.Println("SORTER")
		fmt.Println("Add array manualy or automatically?")
		fmt.Println("(1): Manually; (2): Auto; (0): Exit")

		for {
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
		}

		//TODO thread execution
	}
}

//TODO sorting algorithms

func randArray(nums uint) []int {
	var arr []int

	for i := 0; i < int(nums); i++ {
		arr = append(arr, rand.Intn(600)-100)
	}

	return arr
}
