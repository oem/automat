# automat

## What is it?

It is a tool to help with initial exploratory data analysis on the command line. It is meant to be used together with all the other great tools available.

Specifically, automat provides following functions to help you wrangle with your data:

- [x] filter (can filter numerical values)
- [ ] mutate
- [ ] summarize
- [ ] arrange
- [ ] group_by
- [ ] other, SQL-like operations on your tabular data

## Why?

## Usage

The csv used in all the examples and benchmarks is the worldcitiespop dataset from the [Data Science Toolkit](https://github.com/petewarden/dstkdata).

### Filter

Simple filtering:

`atm worldcitiespop.csv filter "Population<1000000"`

Multiple filter commands can be chained together:

`atm worldcitiespop.csv filter "Population<1000000"|atm filter "Longitude<-50"`

`atm` tries to be a good unix citizen. Use it with other commandline tools, like [xsv](https://github.com/BurntSushi/xsv) for example:

`atm worldcitiespop.csv filter "Population<20"|atm filter "Population>=10"|atm filter "Longitude<-50"|xsv select City,Population|xsv table`

## Setup

If you have [rustup](https://rustup.rs) installed on your system you can simply run `cargo install automat`.

You can also run it via docker.

If you for example have a csv file called worldcitiespop.csv in your local directory:

`docker run --rm -v $(PWD):/data oembot/automat  ./atm /data/worldcitiespop.csv filter "Population<10"`

## Benchmarks

The benchmarks have been created with [hyperfine](https://github.com/sharkdp/hyperfine).

### v.0.0.5

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `target/release/atm test.csv filter "Population<1000000"` | 1.352 ± 0.009 | 1.341 | 1.366 | 1.00 |

### v0.0.4

| Command                                                   |      Mean [s] | Min [s] | Max [s] | Relative |
| :-------------------------------------------------------- | ------------: | ------: | ------: | -------: |
| `target/release/atm test.csv filter "Population<1000000"` | 1.360 ± 0.010 |   1.346 |   1.379 |     1.00 |
