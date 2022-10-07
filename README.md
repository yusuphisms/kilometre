# kilometre
This repo has absolutely nothing to do with units of measurements.

## Generate Test Files

Use `yes` to generate some nice test files. Could go with something like:
```shell
echo "header,columns,go,here" > testfile
yes 'repeated,values,4,rows' | head -n 1000 >> testfile
```