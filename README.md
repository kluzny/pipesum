# pipesum

Sum a column of output from stdin

## Usage

```bash
 $ echo -e '-1\n0\n3.14' | pipesum
 2.14
```

## Options

```bash
$ pipesum --help
Sum a column of output from stdin

Usage: pipesum [OPTIONS]

Options:
  -c, --column <COLUMN>  Column number to sum, 0-indexed [default: 0]
  -w, --warn             warnings to stdout
  -d, --debug            debug to stdout
  -h, --help             Print help
  -V, --version          Print version
```

## Installation

* `git clone https://github.com/kluzny/pipesum`
* `cd pipesum`
* `cargo install --path .`

## TODO

* GPL 3.0
* Github
* have a fail fast mode that non-zero exist on first warn/error
* option to strip out non-numerics ^[\-0-9\.]
* accept user regexes? or better just to chain after grep?
* have an -a, --all that sums every column
* use a non-blocking model via tokio