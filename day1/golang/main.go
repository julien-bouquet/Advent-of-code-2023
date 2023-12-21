package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
)

const REGEX_EXTRACT_NUMBER = "(\\d)"

func main() {
	file, err := os.Open("../input.txt")
	if err != nil {
		fmt.Printf("Error opening file: %v", err)
		os.Exit(1)
	}
	defer file.Close()

	regex := regexp.MustCompile(REGEX_EXTRACT_NUMBER)
	scanner := bufio.NewScanner(file)
	var result = 0
	for scanner.Scan() {
		a := regex.FindAllString(scanner.Text(), -1)
		var first, _ = strconv.Atoi(a[0])
		var last int
		if len(a) > 1 {
			last, _ = strconv.Atoi(a[len(a)-1])
		} else {
			last, _ = strconv.Atoi(a[0])
		}
		result = result + (first*10 + last)
	}
	fmt.Println(result)
}
