# Fluent Bit Lua Tester

A tool to test Lua scripts written for Fluent Bit filters. It calls the function and expects return values using the same interface as Fluent Bit does, using LuaJIT like Fluent Bit does as well.

## Getting Started

This tool is a very early experiment, so it doesn't have any release infrastructure. The only way to run it currently is to have Rust installed and build/run it with `cargo`.

The tool accepts the path to the yaml test config as the first argument to the program.

## Example

Let's test an extremely simple Lua script `example.lua` that we are using as a Fluent Bit Filter:

```lua
function filter_entry(tag, timestamp, record)
    record["y"] = "z"
    return 0, timestamp, record
end
```

The test config yaml file specifies a list of Lua scripts to test, in each:
* The file path (relative to the working directory of the binary)
* The function in the script to call
* An array of tests each with
    - Test case name
    - The input arguments (tag, timestamp, and record)
    - The expected output result (code, timestamp, and record)

Let's write a small set of unit tests for this script in `example_test.yaml`:

```yaml
scripts:
  - file: "example.lua"
    call: "filter_entry"
    tests:
    - name: "adds y key"
      input: 
        tag: "hi"
        timestamp: "2014-10-02T15:01:23Z"
        record:
          w: x
      expected:
        code: 0
        timestamp: "2014-10-02T15:01:23Z"
        record:
          w: x
          y: z
    - name: "resets existing y key"
      input: 
        tag: "hi"
        timestamp: "2014-10-02T15:01:23Z"
        record:
          y: something else
      expected:
        code: 0
        timestamp: "2014-10-02T15:01:23Z"
        record:
          y: z
```

Run this test with the command `cargo run -- example_test.yaml`

```
Running test: "adds y key"
Test Passed

Running test: "resets existing y key"
Test Passed
```