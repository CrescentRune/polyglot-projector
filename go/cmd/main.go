package main

import (
	"fmt"
	"log"

	projector "hank-krutulis.com/go-projector/pkg/cli"
)

func main() {
    opts, err := projector.GetOptions()
    if err != nil {
        log.Fatalf("unable to get options %v", err)
    }

    fmt.Printf("opts: %+v", opts)
}
