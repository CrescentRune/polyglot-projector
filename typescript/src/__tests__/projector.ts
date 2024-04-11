import Projector from "../projector";
import { Operation } from "../config";

function getData() {
    return {
        projector: {
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
        }
    }
}

function getProjector(pwd: string, data = getData()): Projector {
   return new Projector({
        args: [],
        operation: Operation.Print,
        pwd,
        config: "What is up"
   }, data);
}

test("getValueAll", function() {
    const proj = getProjector("/foo/bar");
    expect(proj.getValueAll()).toEqual({
        "baz": "bat",
        "foo": "bar3",
    });
});

test("getValue", function() {
    let proj = getProjector("/foo/bar");
    expect(proj.getValue("foo")).toEqual("bar3");
    proj = getProjector("/foo");
    expect(proj.getValue("foo")).toEqual("bar2");
    expect(proj.getValue("baz")).toEqual("bat");
});

test("setValue", function() {
    let data = getData();
    let proj = getProjector("/foo/bar", data);
    proj.setValue("foo", "baz");
    expect(proj.getValue("foo")).toEqual("baz");


    proj.setValue("baz", "bat3");
    expect(proj.getValue("baz")).toEqual("bat3");

    proj = getProjector("/", data);
    expect(proj.getValue("baz")).toEqual("bat");
});


test("deleteValue", function() {
    let data = getData();
    let proj = getProjector("/foo", data);
    proj.removeValue("baz");
    expect(proj.getValue("baz")).toEqual("bat");

    proj.removeValue("foo");
    expect(proj.getValue("foo")).toEqual("bar1");

    proj = getProjector("/foo/bar", data);
    expect(proj.getValue("foo")).toEqual("bar3");
    proj.removeValue("foo");
    expect(proj.getValue("foo")).toEqual("bar1");

});

