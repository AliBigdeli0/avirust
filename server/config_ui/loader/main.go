package main

import (
	"fmt"
	"net/http"
)

func main() {
	println("going to listen on port:", 9001)
	err := http.ListenAndServe(":9001", http.FileServer(http.Dir("./dist")))
	if err != nil {
		fmt.Println("failed to load server: ", err)
		return
	}
}
