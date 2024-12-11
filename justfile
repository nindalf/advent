CURRENT_YEAR := env_var_or_default("AOC_YEAR", "2024")

default:
    @just --list

# Fetch input for a specific day in the current year
fetch DAY:
    @cd y{{CURRENT_YEAR}} && aocgen --day {{DAY}} --year {{CURRENT_YEAR}}

# Benchmark a specific day in a specific year
bench DAY="":
    #!/usr/bin/env sh
    if [ "{{DAY}}" = "" ]; then
        @cargo bench --manifest-path y{{CURRENT_YEAR}}/Cargo.toml
    else
        @cargo bench --manifest-path y{{CURRENT_YEAR}}/Cargo.toml -- day{{DAY}}
    fi

# Run tests for a specific day, optionally targeting a specific part
test DAY="" TARGET="":
    #!/usr/bin/env sh
    if [ "{{DAY}}" = "" ]; then
        @cargo test --manifest-path y{{CURRENT_YEAR}}/Cargo.toml
    else
        @cargo test --manifest-path y{{CURRENT_YEAR}}/Cargo.toml -- day{{DAY}}::tests::part_{{TARGET}}
    fi
  
# Refetch input for a specific day in a specific year
refetch DAY:
    @rm -f y{{CURRENT_YEAR}}/src/day{{DAY}}/Readme.md
    @cd y{{CURRENT_YEAR}} && aocgen --day {{DAY}} --year {{CURRENT_YEAR}}
