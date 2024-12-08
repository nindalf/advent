default:
  @just --list

fetch DAY:
  @aocgen --day {{DAY}}

bench DAY:
  @cargo bench -- day{{DAY}}

test-all:
  @cargo test

test DAY TARGET="":
  @cargo test -- day{{DAY}}::tests::part_{{TARGET}}

refetch DAY:
  @rm src/day{{DAY}}/Readme.md
  @aocgen --day {{DAY}}
