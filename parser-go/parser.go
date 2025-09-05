package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strconv"
	"strings"
)

type LogData struct {
	DirName string
	Pub     string
	Sub     string
}

func parseLogFile(filePath string) string {
	content, err := ioutil.ReadFile(filePath)
	if err != nil {
		return "N/A"
	}

	re := regexp.MustCompile(`(\d{1,3}(?:,\d{3})*|\d+)\s+msgs/sec`)
	match := re.FindStringSubmatch(string(content))
	if len(match) > 1 {
		return strings.ReplaceAll(match[1], ",", "")
	}
	return "N/A"
}

func getSortKey(dirName string) int {
	lowerDirName := strings.ToLower(dirName)
	if strings.HasSuffix(lowerDirName, "ms") {
		val, _ := strconv.Atoi(strings.TrimSuffix(lowerDirName, "ms"))
		return val
	} else if strings.HasSuffix(lowerDirName, "s") {
		val, _ := strconv.Atoi(strings.TrimSuffix(lowerDirName, "s"))
		return val * 1000
	} else if strings.HasSuffix(lowerDirName, "m") {
		val, _ := strconv.Atoi(strings.TrimSuffix(lowerDirName, "m"))
		return val * 60 * 1000
	}
	return int(^uint(0) >> 1) // Max int
}

func main() {
	var data []LogData

	files, err := ioutil.ReadDir(".")
	if err != nil {
		log.Fatal(err)
	}

	for _, f := range files {
		if f.IsDir() {
			dirName := f.Name()
			pubLogPath := filepath.Join(dirName, "bench-pub.log")
			subLogPath := filepath.Join(dirName, "bench-sub.log")

			if _, err := os.Stat(pubLogPath); !os.IsNotExist(err) {
				if _, err := os.Stat(subLogPath); !os.IsNotExist(err) {
					pubVal := parseLogFile(pubLogPath)
					subVal := parseLogFile(subLogPath)
					data = append(data, LogData{DirName: dirName, Pub: pubVal, Sub: subVal})
				}
			}
		}
	}

	sort.Slice(data, func(i, j int) bool {
		return getSortKey(data[i].DirName) < getSortKey(data[j].DirName)
	})

	fmt.Println("| Sync Period | Pub r/s | Sub r/s |")
	fmt.Println("|---|---|---|")
	for _, d := range data {
		fmt.Printf("| %s | %s | %s |\n", d.DirName, d.Pub, d.Sub)
	}
}
