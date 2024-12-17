package main

import (
	"fmt"
	"math"
	"os"
	"path/filepath"
	"sort"
	"strconv"
	"strings"
)

func main() {
	topElfCount()
	topThreeElfCount()
}

func topThreeElfCount() {
	path, _ := filepath.Abs("input")
	content, err := os.ReadFile(path)
	if err != nil {
		panic(err)
	}
	elfGroups := strings.Split(string(content), "\r\n\r\n")
	var sortedElfGroups []int

	for _, group := range elfGroups {
		groupSlice := strings.Split(group, "\r\n")
		groupTotal := sumGroup(groupSlice)
		sortedElfGroups = append(sortedElfGroups, groupTotal)
	}
	sort.Slice(sortedElfGroups, func(i, j int) bool {
		return sortedElfGroups[i] > sortedElfGroups[j]
	})
	topThreeSum := sum(sortedElfGroups[:3])
	fmt.Println(topThreeSum)
}

func sum(array []int) int {
	var result int
	for _, i := range array {
		result += i
	}
	return result
}

func topElfCount() {
	path, _ := filepath.Abs("input")
	content, err := os.ReadFile(path)
	if err != nil {
		panic(err)
	}
	elfGroups := strings.Split(string(content), "\r\n\r\n")
	var maxCalories int
	for _, group := range elfGroups {
		groupSlice := strings.Split(group, "\r\n")
		groupTotal := sumGroup(groupSlice)
		maxCalories = int(math.Max(float64(maxCalories), float64(groupTotal)))
	}
	fmt.Println(maxCalories)
}

func sumGroup(group []string) int {
	var sum int
	for _, item := range group {
		if item == "" {
			continue
		}
		intItem, err := strconv.Atoi(item)
		if err != nil {
			panic(err)
		}
		sum += intItem
	}
	return sum
}
