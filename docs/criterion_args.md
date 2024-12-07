# Criterion.rs "0.5.1" Arguments Testing

## Arguments that FAIL

### `error: unexpected argument found`

- `--help`

  ```console
  cargo bench -p SUB_CRATE_USING_CRITERION -- --help
  ```

- `-h`

  ```console
  cargo bench -p SUB_CRATE_USING_CRITERION -- -h
  ```

- `--message-format=json` from [Criterion.rs Documentation](https://bheisler.github.io/criterion.rs/book/cargo_criterion/external_tools.html)

  ```console
  cargo bench -p SUB_CRATE_USING_CRITERION -- --message-format=json
  ```

### `error: one of the values isn't valid for an argument`

- `--format json`

  ```console
  cargo bench -p SUB_CRATE_USING_CRITERION -- --format json
  ```

- `--format=json`

  ```console
  cargo bench -p SUB_CRATE_USING_CRITERION -- --format=json
  ```

### `error: an argument cannot be used with one or more of the other specified arguments`

- `--bench`

  ```console
  cargo bench -p SUB_CRATE_USING_CRITERION -- --bench
  ```

## Arguments that SUCCEED

- `--verbose`

  ```console
  cargo bench -p SUB_CRATE_USING_CRITERION -- --verbose
  ```

- `--list`

  ```console
  cargo bench -p SUB_CRATE_USING_CRITERION -- --list
  ```

## NO more help messages even with CRITERION_DEBUG=1

- `CRITERION_DEBUG=1`

  ```console
  CRITERION_DEBUG=1 cargo bench ...
  ```
