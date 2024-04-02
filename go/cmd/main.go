package main

import (
	"fmt"
	"log"
    "hank-krutulis.com/go-projector/pkg/projector"
)

func main() {
    opts, err := projector.GetOptions()
    if err != nil {
        log.Fatalf("unable to get options %v", err)
    }

    config, err := projector.NewConfig(opts)
    if err != nil {
        log.Fatalf("unable to get config: %v", err)
    }

    fmt.Printf("opts: %+v", opts)
    fmt.Printf("config: %+v", config)
}
