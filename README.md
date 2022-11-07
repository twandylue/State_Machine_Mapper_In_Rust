# State_Machine_Mapper_In_Rust

Generate two dimension array for state machine.

## Quick Start

Prepare data in `input.csv`.

* In input.csv

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
states and commands map:
[[1, 2, -1], [2, 3, -1], [-1, -1, 3], [-1, -1, -1]]

* Output:
  command_1  command_2  command_3
[
  [1, 2, -1], | state_a
  [2, 3, -1], | state_b
  [-1, -1, 3], | state_c
  [-1, -1, -1], | state_d
]
```

## References

* [Using state machine to validate number](https://blog.csdn.net/kenden23/article/details/18696083)
* [Finite-state machine](https://zh.wikipedia.org/zh-tw/%E6%9C%89%E9%99%90%E7%8A%B6%E6%80%81%E6%9C%BA)
