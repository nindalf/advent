AOC_YEAR := env_var_or_default("AOC_YEAR", "2023")

# Print all commands
default:
    @echo "Advent of Code {{AOC_YEAR}}"
    @just --list

# Fetch test input and create
fetch DAY:
    @cd y{{AOC_YEAR}} && aocgen fetch --day {{DAY}} --year {{AOC_YEAR}}

# Refetch readme for a specific day
# The problem changes after the second part is unlocked
refetch DAY:
    @rm -f y{{AOC_YEAR}}/src/day{{DAY}}/Readme.md
    @cd y{{AOC_YEAR}} && aocgen fetch --day {{DAY}} --year {{AOC_YEAR}}

submit DAY PART ANSWER:
    @aocgen submit --year {{AOC_YEAR}} --day {{DAY}} --part {{PART}} --answer {{ANSWER}}

# Run benchmarks for the year or a specific day
bench DAY="":
    #!/usr/bin/env sh
    if [ "{{DAY}}" = "" ]; then
        cargo bench -- y{{AOC_YEAR}}
    else
        # The extra space after day{{DAY}} is needed to avoid `just bench 1` matching with 11, 12...
        cargo bench -- "y{{AOC_YEAR}} day{{DAY}} "
    fi

# Run tests for the year or a specific day, optionally targeting a specific part
test DAY="" TARGET="":
    #!/usr/bin/env sh
    if [ "{{DAY}}" = "" ]; then
        cargo test --manifest-path y{{AOC_YEAR}}/Cargo.toml
    else
        cargo test --manifest-path y{{AOC_YEAR}}/Cargo.toml -- day{{DAY}}::tests::part_{{TARGET}}
    fi
