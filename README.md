# automat

## What is it?

It is a tool to help with initial exploratory data analysis on the command line. It is meant to be used together with all the other great tools available.

Specifically, automat provides following functions to help you wrangle with your data:

- [ ] filter :toolbox: (in progress)
- [ ] mutate
- [ ] summarize
- [ ] arrange
- [ ] group_by
- [ ] other, SQL-like operations on your tabular data

## Why?

## Usage

## Setup

## Benchmarks

The csv used in the benchmarks is the worldcitiespop dataset from the [Data Science Toolkit](https://github.com/petewarden/dstkdata).

### v0.0.4

| Command                                                   |      Mean [s] | Min [s] | Max [s] | Relative |
| :-------------------------------------------------------- | ------------: | ------: | ------: | -------: |
| `target/release/atm test.csv filter "Population<1000000"` | 1.360 ± 0.010 |   1.346 |   1.379 |     1.00 |
