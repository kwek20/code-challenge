## Goal
Takes in CSV of transactions,
runs throuhg a transactions engine
output the client account balances

Run the binary: `cargo run -- transactions.csv > accounts.csv`

## Build info
clippy requires cargo nightly to run
`cargo +nightly clippy`

fmt also requires cargo nightly to run for some of the selected features
`cargo +nightly clippy`

Running the example requires either a passed argument for the input file, or set in the .env during development build. 
Release build doesnt use .env

## Environment variables
Environment variables are only active in debug mode.
We read the following:

Input file is used in debug mode, when no parameter has been passed.
Resource folder is not related to this, hence it being prepended here.
`INPUT_FILE="resources/transactions.csv"`

Resource folder is used when a parameter is passed to the binary
`RESOURCE_FOLDER="resources"`

## Logging
Due to the requirements of the challenge outputting the result to stdout, we log informative messages to stderr. 