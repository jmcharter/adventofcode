package main

import (
	"bufio"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"strings"
)

func main() {
	partOne()
	partTwo()

}

func partOne() {
	file, scanner := getFileScanner("input")
	defer file.Close()
	var sum int
	for scanner.Scan() {
		var duplicates string
		bisected_strings := bisectString(scanner.Text())
		for _, c1 := range bisected_strings[0] {
			if strings.ContainsRune(bisected_strings[1], c1) && !strings.ContainsRune(duplicates, c1) {
				duplicates += string(c1)
				val, _ := getCharValue(c1)
				sum += val
			}
		}
	}
	println(sum)
}

func partTwo() {
	file, scanner := getFileScanner("input")
	defer file.Close()
	var sum int
	var count int
	var group []string
	for scanner.Scan() {
		var duplicates string
		count += 1
		mod := count % 3
		if mod == 1 {
			group = nil
		}

		group = append(group, scanner.Text())
		if !(mod == 0) {
			continue
		}
		for _, r := range group[0] {
			if strings.ContainsRune(group[1], r) && strings.ContainsRune(group[2], r) && !strings.ContainsRune(duplicates, r) {
				duplicates += string(r)
				value, _ := getCharValue(r)
				sum += value
			}
		}

	}
	println(sum)
}

// Returns an integer value for the first character of a string
// a-z = 1-26
// A-Z = 27 - 52
func getCharValue(r rune) (int, error) {
	s := string(r)
	var err error = nil
	isAlpha := regexp.MustCompile(`^[a-zA-Z]+$`).MatchString(s)
	if !isAlpha {
		err = fmt.Errorf("getCharValue requires a string only containing alphabetic characters (a-zA-Z), got '%s'", s)
	}
	if err != nil {
		return 0, err
	}
	var value int
	capital := false
	if s == strings.ToUpper(s) {
		capital = true
		s = strings.ToLower(s)
	}
	c := []rune(s)[0]
	if capital {
		value = int(c - 70)
	} else {
		value = int(c - 96)
	}
	return value, err
}

func bisectString(s string) []string {
	halfLength := len(s) / 2
	partOne := s[:halfLength]
	partTwo := s[halfLength:]
	return []string{partOne, partTwo}
}

func getFileScanner(filename string) (*os.File, *bufio.Scanner) {
	path, err := filepath.Abs(filename)
	if err != nil {
		panic(err)
	}
	file, err := os.Open(path)
	if err != nil {
		panic(err)
	}
	scanner := bufio.NewScanner(file)
	return file, scanner
}
