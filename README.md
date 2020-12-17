# Gotcha

Gotcha is CLI tool for generate random string.

## How to install

Install gotcha from source.

You have to install Rust and Cargo.

```
git clone https://github.com/nnnamani/gotcha.git
cd gotcha
cargo install --path .
```

## How to uninstall

```
cd gotcha
cargo uninstall
```

## Test

```
cargo test
```

## Usage

- Generate random string
  ```
  $ gotcha
  Qswu#=(6b<
  ```

- Generate random string specified length
  ```
  $ gotcha -l 20
  lG1c<ho/daB-z<eSZ}7&
  ```

- Generate random string includes number only
  ```
  $ gotcha -n
  5932798973
  ```

- Generate random string includes alphabet only
  ```
  $ gotcha -aA
  mVVJXFThMo
  # lower case
  $ gotcha -a
  viicidsfsu
  # upper case
  $ gotcha -A
  YSUNCMHTHM
  ```

- help
  ```
  $ gotcha -h
  gotcha 0.1.0
  nnnamani <6526114+nnnamani@users.noreply.github.com>
  Random string generate tool

  USAGE:
      gotcha [FLAGS] [OPTIONS]

  FLAGS:
      -h, --help              Prints help information
      -a, --with-lowercase    With lowercase letter [a - z]
      -n, --with-number       With number string [0 - 9]
      -s, --with-symbol       With symbol [!#$%&'()*+,-./;<=>?@[]^_`{|}~]
      -A, --with-uppercase    With uppercase letter [A - Z]
      -V, --version           Prints version information

  OPTIONS:
      -l, --length <LENGTH>    Length of generated string
  ```
