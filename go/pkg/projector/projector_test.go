package projector_test

import (
	"testing"
	"hank-krutulis.com/go-projector/pkg/projector"
)

func getData() *projector.Data {
    return &projector.Data{
        Projector: map[string]map[string]string{
            "/": {
                "foo": "bar1",
                "baz": "bat",
            },
            "/foo": {
                "foo": "bar2",
            },
            "/foo/bar": {
                "foo": "bar3",
            },
        },
    }
}

func getProjector(pwd string, data *projector.Data) *projector.Projector {
    return projector.CreateProjector(
        &projector.Config{
            Args: []string{},
            Operation: projector.Print,
            Pwd: pwd,
            Config: "What is up",
        }, 
        data,
    ); 
}

func test(t *testing.T, proj *projector.Projector, key, value string) {
    v, ok := proj.GetValue(key)
    if !ok {
        t.Errorf("expected to find value \"%v\"", value)
    }

    if v != value {
        t.Errorf("expected to find %v but received %v", value, v)
    }
}

func TestGetValue(t *testing.T) {
    data := getData()
    projector := getProjector("/foo/bar", data)
    test(t, projector, "foo", "bar3")
    test(t, projector, "baz", "bat")
}

func TestSetValue(t *testing.T) {
    data := getData()
    projector := getProjector("/foo/bar", data)
    test(t, projector, "foo", "bar3")

    projector.SetValue("foo", "bar4")
    test(t, projector, "foo", "bar4")
    
    projector.SetValue("baz", "buzz")
    test(t, projector, "baz", "buzz")

    projector = getProjector("/foo", data)
    test(t, projector, "foo", "bar2")
    test(t, projector, "baz", "bat")

}


func TestRemoveValue(t *testing.T) {
    data := getData()
    projector := getProjector("/foo/bar", data)
    test(t, projector, "foo", "bar3")
    projector.RemoveValue("foo")
    test(t, projector, "foo", "bar2")

    projector.RemoveValue("baz")
    test(t, projector, "baz", "bat")
}
