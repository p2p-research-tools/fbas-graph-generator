# FBAS graph generator

![MIT](https://img.shields.io/badge/license-MIT-blue.svg)
[![Build](https://github.com/p2p-research-tools/fbas-graph-generator/actions/workflows/test.yml/badge.svg)](https://github.com/p2p-research-tools/fbas-graph-generator/actions/workflows/test.yml)
[![codecov](https://codecov.io/gh/p2p-research-tools/fbas-graph-generator/branch/main/graph/badge.svg?token=C1FPEQU21W)](https://codecov.io/gh/p2p-research-tools/fbas-graph-generator)
[![dependency status](https://deps.rs/repo/github/p2p-research-tools/fbas-graph-generator/status.svg)](https://deps.rs/repo/github/p2p-research-tools/fbas-graph-generator)

Generate (weighted) trust graphs for FBASs like [Stellar](https://www.stellar.org/).

The binary reads an FBAS in [stellarbeat](https://stellarbeat.io/)'s JSON format and

- ranks the nodes using the [fbas-rewards-distributor](https://github.com/p2p-research-tools/fbas-reward-distributor) if instructed to
- writes 2 files containing commonly used graph encodings, i.e.
    - a nodes list with optional weights (using one of the algorithms implemented in the above tool) for each node and
    - an adjacency matrix where A(u, v) = 1 signifies the precense of the edge u->v.

All nodes with 'unsatisfiable' quorum sets are not included in the output and nodes marked as inactive can optionally be excluded from the output.

## Output Format

The data written by the tool adheres to convential graph writing formats used by popular graph analysis tools such as Gephi and NetworkX.

## Using

Run the following for usage instructions:

```
cargo run --release -- -h
```

## Example

The command

```
cargo run --release -- test_data/mobilecoin_nodes_2021-10-22.json -i -p -o example_output power-index-enum
```

ranks the nodes in the FBAS and creates the following two files in the `example_output/` directory

```
mobilecoin_nodes_2021-10-22_power_index_enum_nodelist.csv
mobilecoin_nodes_2021-10-22_power_index_enum_adjacency_matrix.csv
```
