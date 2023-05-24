package main

import (
	"fmt"
	"github.com/davidbyttow/govips/v2/vips"
    "io/ioutil"
    "time"
)

func main() {
	// List of files to generate thumbnails from
	files := []string{
        "../data/1.jpg",
        "../data/2.jpg",
        "../data/3.jpg",
        "../data/4.jpg",
        "../data/5.jpg",
	}
    vips.Startup(nil)
	defer vips.Shutdown()

	// Loop through the files and generate thumbnails
	for _, file := range files {
		start := time.Now()
		thumb, err := vips.NewThumbnailFromFile(file, 320, 320, vips.InterestingNone)
		if err != nil {
			fmt.Printf("Error opening file: %s\n", file)
			continue
		}
        ep := vips.NewDefaultJPEGExportParams()
        thumbbytes, _, err := thumb.Export(ep)
        err = ioutil.WriteFile("output.jpg", thumbbytes, 0644)

        elapsed := time.Since(start).Milliseconds()
        fmt.Printf("Thumbnail (Time: %d ms)\n", elapsed)

	}
}