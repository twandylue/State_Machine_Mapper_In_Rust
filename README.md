# State_Machine_Mapper_In_Rust

Transform One-dimension State-transition table to Two-dimensions State-transition table for state machine.

## Introduction

Prepare data in `input.csv`.  

In `input.csv`, data should be presented in One-dimension State-transition table in csv format.

| Current state   | Command       | Next state     |
| --------------- | ------------- | -------------- |
| state_a         | command_1     | state_b        |
| state_a         | command_2     | state_c        |
| state_b         | command_1     | state_c        |
| state_b         | command_2     | state_d        |
| state_c         | command_3     | state_d        |

```csv
state_a, command_1, state_b
state_a, command_2, state_c
state_b, command_1, state_c
state_b, command_2, state_d
state_c, command_3, state_d
```

Then, stdout will show Two-dimensions State-transition table (in 2D array) in different languages, including `rust`, `cpp`, `js`, `python`, `c_sharp` and `go`.

| --              | command_1(0)  | command_2(1)   | command_3(2)   |
| --------------- | ------------- | -------------- | -------------- |
| state_a(0)      | state_b(1)    | state_c(2)     | None(-1)       |
| state_b(1)      | state_c(2)    | state_d(3)     | None(-1)       |
| state_c(2)      | None(-1)      | None(-1)       | state_d(3)     |
| state_d(3)      | None(-1)      | None(-1)       | None(-1)       |

```console
  command_1  command_2  command_3
[
  [1, 2, -1], | state_a
  [2, 3, -1], | state_b
  [-1, -1, 3], | state_c
  [-1, -1, -1], | state_d
]
```

## Quick Start

**In `input.csv`**

  ```csv
  state_a, command_1, state_b
  state_a, command_2, state_c
  state_b, command_1, state_c
  state_b, command_2, state_d
  state_c, command_3, state_d
  ```

After that, let's go!

```console
$ cargo run -- input.csv
states list: {
    "state_a": 0,
    "state_b": 1,
    "state_c": 2,
    "state_d": 3,
}
commands list: {
    "command_1": 0,
    "command_2": 1,
    "command_3": 2,
}

* === 2D State-transition table ===
  command_1  command_2  command_3
[
  [1, 2, -1], | state_a
  [2, 3, -1], | state_b
  [-1, -1, 3], | state_c
  [-1, -1, -1], | state_d
]

* === 2D State-transition table(2D array) in different languages ===
Map in rust:
let array = [[1, 2, -1], [2, 3, -1], [-1, -1, 3], [-1, -1, -1]];
Map in cpp:
int array[4][3] = {{1, 2, -1}, {2, 3, -1}, {-1, -1, 3}, {-1, -1, -1}};
Map in go:
var array = [4][3]int{{1, 2, -1}, {2, 3, -1}, {-1, -1, 3}, {-1, -1, -1}}
Map in c_sharp:
int [,] array = new int[,] {{1, 2, -1}, {2, 3, -1}, {-1, -1, 3}, {-1, -1, -1}};
Map in js:
const array = [[1, 2, -1], [2, 3, -1], [-1, -1, 3], [-1, -1, -1]];
Map in python:
const array = [[1, 2, -1], [2, 3, -1], [-1, -1, 3], [-1, -1, -1]]
```

## References

* [Using state machine to validate number](https://blog.csdn.net/kenden23/article/details/18696083)
* [Finite-state machine](https://zh.wikipedia.org/zh-tw/%E6%9C%89%E9%99%90%E7%8A%B6%E6%80%81%E6%9C%BA)
* [State-transition table](https://en.wikipedia.org/wiki/State-transition_table)
