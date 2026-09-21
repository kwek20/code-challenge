## Goal
Takes in CSV of transactions,
runs throuhg a transactions engine
output the client account balances

## Build info
clippy requires cargo nightly to run
`cargo +nightly clippy`

fmt also requires cargo nightly to run for some of the selected features
`cargo +nightly clippy`

Running the example requires either a passed argument for the input file, or set in the .env during development build. 
Release build doesnt use .env