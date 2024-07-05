# Personal Learning Project - wc
Further learning by attempting to recreate unix `wc`

## Challenge
The idea and example testing file (test.txt) for this "challenge" was taken from [codingchallengs.fyi](https://codingchallenges.fyi/challenges/challenge-wc/).

## Installation
I don't plan on turning this into an offical crate so:
```
cargo install --path ./path/to/learning_wc/root/
```
## Uninstall
```
cargo uninstall learning_wc
```

## Usage
```
Usage: learning_wc [OPTIONS] [FILE]...

Arguments:
  [FILE]...  optional files

Options:
  -c, --bytes    print the byte counts
  -l, --lines    print the newline counts
  -w, --words    print the word counts
  -m, --chars    print the character counts
  -h, --help     Print help
  -V, --version  Print version

```