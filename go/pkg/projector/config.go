package projector

import (
	"fmt"
	"os"
	"path"
)

type Operation = int 
const(
    Print Operation = iota
    Add
    Remove
)

type Config struct {
    Args []string
    Operation Operation
    Pwd string
    Config string
}

func getPwd(opts *ProjectorOpts) (string, error) {
    if opts.Pwd != "" {
        return opts.Pwd, nil
    }

    return os.Getwd()
}

func getConfig(opts *ProjectorOpts) (string, error) {
    if opts.Config != "" {
        return opts.Pwd, nil
    }

    config, err := os.UserConfigDir()
    if (err != nil) {
        return "", err
    }
    
    return path.Join(config, "projector", "projector.json"), nil
}

func getOperation(opts *ProjectorOpts) Operation {
    if len(opts.Args) == 0 {
        return Print
    }

    if opts.Args[0] == "add" {
        return Add
    }

    if opts.Args[0] == "delete" {
        return Remove
    }

    return Print
}

func getArgs(opts *ProjectorOpts) ([]string, error) {
    if len(opts.Args) == 0 {
        return []string{}, nil
    }

    operation := getOperation(opts)

    if operation == Add {
        if len(opts.Args) != 3 {
            return nil, fmt.Errorf("Add requires 2 args but received %v", len(opts.Args) - 1)
        }
        return opts.Args[1:], nil
    }

    if operation == Remove {
        if len(opts.Args) != 2 {
            return nil, fmt.Errorf("Add requires 1 args but received %v", len(opts.Args) - 1)
        }
        return opts.Args[1:], nil
    }

    if len(opts.Args) > 1 {
        return nil, fmt.Errorf("Print expects no args but got %v", len(opts.Args) - 1)
    }

    return opts.Args, nil
}

func NewConfig(opts *ProjectorOpts) (*Config, error) {
    pwd, err := getPwd(opts)
    if err != nil {
        return nil, err
    }

    config, err := getConfig(opts)
    if err != nil {
        return nil, err
    }

    args, err := getArgs(opts)
    if err != nil {
        return nil, err
    }

    return &Config{
        Pwd: pwd,
        Config: config,
        Operation: getOperation(opts),
        Args: args,
    }, nil
            

}
