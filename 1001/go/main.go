package main

import (
	"fmt"
)

func main() {
	var a, b int
	_, err := fmt.Scan(&a)
	if err != nil {
		fmt.Println("Error:", err)
		return
	}
	_, err = fmt.Scan(&b)
	if err != nil {
		fmt.Println("Error:", err)
		return
	}

	fmt.Println("X =", a+b)
}
