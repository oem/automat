# automat

## What is it?

It is a tool to help with initial exploratory data analysis on the command line. It is meant to be used together with all the other great tools available.

Specifically, automat provides following functions to help you wrangle with your data:

- [ ] filter (in progress, majority implemented) :toolbox:
- [ ] mutate
- [ ] summarize
- [ ] arrange
- [ ] group_by
- [ ] other, SQL-like operations on your tabular data

## Why?

## Usage

### Filter

Simple filtering:

`atm worldcitiespop.csv filter "Population<1000000"`

Multiple filter commands can be chained together:

`atm worldcitiespop.csv filter "Population<1000000"|atm filter "Longitude<-50"`

`atm` tries to be a good unix citizen. Use it with other commandline tools, like [xsv](https://github.com/BurntSushi/xsv) for example:

`atm test.csv filter "Population<20"|atm filter "Population>=10"|atm filter "Longitude<-50"|xsv select City,Population|xsv table`

## Setup

If you have [rustup](https://rustup.rs) installed on your system you can simply run `cargo install automat`.

## Benchmarks

The csv used in the benchmarks is the worldcitiespop dataset from the [Data Science Toolkit](https://github.com/petewarden/dstkdata).

### v0.0.4

| Command                                                   |      Mean [s] | Min [s] | Max [s] | Relative |
| :-------------------------------------------------------- | ------------: | ------: | ------: | -------: |
| `target/release/atm test.csv filter "Population<1000000"` | 1.360 ± 0.010 |   1.346 |   1.379 |     1.00 |
