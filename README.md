# kilometre
This repo has absolutely nothing to do with units of measurements.

## Running the Binaries
For many if not all of these, I have been using reading from stdin. Example: `cargo run < testfile.csv`

## Generate Test Files

Use `yes` to generate some nice test files. Could go with something like:
```shell
echo "header,columns,go,here" > testfile
yes 'repeated,values,4,rows' | head -n 1000 >> testfile
```