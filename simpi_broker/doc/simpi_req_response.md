# SimPi Request Response Documentation

## General Structure
Generally, the format is the following one:
```
op:{operation_name}
>{status};{key};{value}
```

Any of the `{fields}` above can be omitted. This for instance is a unusual
but nevertheless valid format:
```
op:
>;;
```
Responses can have a theroretical infinte number of returned data triples (lines beginning with `>`), but
must have exactly one `op:` line!

## Field specifications

### operation_name
Is the operation name requested, regardless of it actually exists or not.

### status
- `SUCC`: Successfully completed operation.
- `FAIL`: Failed to complete operation. To describe the failed operation, use
the `FAIL~{CODE}` format (see bullet point below).
- `FAIL~`
  - `UNKAPICALL`: Unknown API Call
  - `UNKACT`: Unknown Action
  - `NYI`: Not Yet Implemented
  - `PNF`: Pin Not Found
  - `PWC`: Pin Wrong Config (Input/Output/Pwm)

### key
The key name of the returned data.

### value
The corresponding value of the key.

## Examples

- `/api/getpin/GPIO18`
    ```
    op:getpin
    >SUCC;GPIO18;1
    ```

- `/api/getpin/GPIO18;GPIO23;GPIO24;GPIO25`
    (the pin values were previously set to 1 0 1 0 in this example!)
    ```
    op:getpin
    >SUCC;GPIO18;1
    >SUCC;GPIO23;0
    >SUCC;GPIO24;1
    >SUCC;GPIO25;0
    ```

- `/api/getpin/GPIO18;99;FOOBAR;GPIO25`
    ```
    op:getpin
    >SUCC;GPIO18;1
    >FAIL~PNF;99;
    >FAIL~PNF;FOOBAR;
    >SUCC;GPIO25;0
    ```

- `/api/hello/123`
    ```
    op:hello
    >FAIL~UNKAPICALL;;
    ```

- `/api/action/foo`
    ```
    op:action
    >FAIL~UNKACT;;
    ```