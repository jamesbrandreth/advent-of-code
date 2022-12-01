# Nathan Samra's Advent of Code

Last year I got about 5 days in and stopped so let's hope this year goes better.

Note all commands in this file are to be run from this folder, i.e ***2022/nathans106***, unless otherwise specified.

## Approach

I'm going to do this as a Rust and Python project. I'm primarily a C++ developer but Rust seems really good to me so that will handle the bulk of the logic. I'll be using Python for input parsing because I've still not found a parser better than PyParsing.

## Dependencies

1. Python 3
2. Rust nightly

## Setup

1. **Optional:** Create and activate a Python virtual environment.
   1. `python -m venv .venv`
   2. `source .venv/Scripts/activate`
2. Install Python dependencies with `pip install -r requirements.txt`
3. Install the commit hooks with `pre-commit install -c .pre-commit-config.yaml`

## Building

1. Compile the Rust with `maturin develop`
