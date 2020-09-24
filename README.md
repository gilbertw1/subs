# Subs

A utility that substitutes variables in templates.

### Installation

Build

    cargo build --release

Run

    ./target/release/subs

Example

    # Read mappings from 'mappings.txt'
    # Use it to substitue variables in 'test.txt'
    # Write result to stdout.
    ./subs --mappings-file mappings.txt test.txt -

### Usage

```
subs 0.1.0
Bryan G. <bryan@bryan.sh>
A utility that can be used to substitute variables in templates.

USAGE:
    subs [OPTIONS] [ARGS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -e, --end-delimiter <DELIMITER>      Delimiter used to delineate the end of a substitution. [default: }}]
        --mappings <MAPPINGS>            Replacement mappings definitions [env: SUBS_MAPPINGS=]
        --mappings-file <FILE>           File containing replacement mappings definitions. Overrides mappings argument.
                                         Use '-' for stdin.
    -s, --start-delimiter <DELIMITER>    Delimiter used to delineate the start of a substitution. [default: {{]

ARGS:
    <IN-FILE>     Template file to be used. Use '-' for stdin.
    <OUT-FILE>    File to write resulting template substitution out to. Use '-' for stdout.
```
