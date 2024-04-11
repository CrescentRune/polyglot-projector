import getConfig, { Operation } from '../config';

test("simple print all", function() {
    const config = getConfig({});

    expect(config.operation).toEqual(Operation.Print);
    expect(config.args).toEqual([]);
});

test('simple print key', function() {
    const config = getConfig({
        args: ["foo"]
    });

    expect(config.operation).toEqual(Operation.Print);
    expect(config.args).toEqual(["foo"]);
});

test('simple add key', function() {
    const config = getConfig({
        args: ["add", "foo", "bar"]
    });

    expect(config.operation).toEqual(Operation.Add);
    expect(config.args).toEqual(["foo", "bar"]);
});

test('simple rm key', function() {
    const config = getConfig({
        args: ["delete", "foo"]
    });

    expect(config.operation).toEqual(Operation.Remove);
    expect(config.args).toEqual(["foo"]);
});
