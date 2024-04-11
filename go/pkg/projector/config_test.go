package projector_test

import (
	"reflect"
	"testing"

	"hank-krutulis.com/go-projector/pkg/projector"
)

func getOpts(args []string) *projector.ProjectorOpts {
    opts := &projector.ProjectorOpts {
        Args: args,
        Config: "",
        Pwd: "",
    }

    return opts
}

func TestConfigPrint(t *testing.T) {
    testConfig(t, []string{}, projector.Print, []string{})
}

func TestConfigPrintKey(t *testing.T) {
    testConfig(t, []string{"foo"}, projector.Print, []string{"foo"})
}

func testConfig(t *testing.T, args []string, operation projector.Operation, expectedArgs []string) {
    opts := getOpts(args)
    config, err := projector.NewConfig(opts)
        
    if err != nil {
       t.Errorf("expected no error %v", err) 
    }

    if config.Operation != operation {
        t.Errorf("operation expected: %v, actual: %v", operation, config.Operation)
    }

    if !reflect.DeepEqual(expectedArgs, config.Args) {
        t.Errorf("expected args to be %v was %v", expectedArgs, config.Args)
    }
}

func TestConfigAddKeyValue(t *testing.T) {
    testConfig(t, []string{"add", "foo", "bar"}, projector.Add, []string{"foo", "bar"})
}

func TestConfigDeleteKey(t *testing.T) {
    testConfig(t, []string{"delete", "foo"}, projector.Remove, []string{"foo"})
}

