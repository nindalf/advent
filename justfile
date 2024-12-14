AOC_YEAR := env_var_or_default("AOC_YEAR", "2024")

default:
    @just --list

# Fetch input for a specific day in the current year
fetch DAY:
    @cd y{{AOC_YEAR}} && aocgen --day {{DAY}} --year {{AOC_YEAR}}

# Benchmark a specific day in a specific year
bench DAY="":
    #!/usr/bin/env sh
    if [ "{{DAY}}" = "" ]; then
        cargo bench
    else
        cargo bench -- day{{DAY}}
    fi

# Run tests for a specific day, optionally targeting a specific part
test DAY="" TARGET="":
    #!/usr/bin/env sh
    if [ "{{DAY}}" = "" ]; then
        cargo test --manifest-path y{{AOC_YEAR}}/Cargo.toml
    else
        cargo test --manifest-path y{{AOC_YEAR}}/Cargo.toml -- day{{DAY}}::tests::part_{{TARGET}}
    fi
  
# Refetch input for a specific day in a specific year
refetch DAY:
    @rm -f y{{AOC_YEAR}}/src/day{{DAY}}/Readme.md
    @cd y{{AOC_YEAR}} && aocgen --day {{DAY}} --year {{AOC_YEAR}}
