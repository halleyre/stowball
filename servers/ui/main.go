package main

import (
	"log"
	"net/http"
)

func main() {
	fs := http.FileServer(http.Dir("./client"))
	http.Handle("/", fs)

	log.Println("starting handler (:8000)")
	if err := http.ListenAndServe(":8000", nil); err != nil {
		log.Fatal(err)
	}
}
