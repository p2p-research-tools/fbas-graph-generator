# FBAS graph generator

Generate trust graphs for FBASs like [Stellar](https://www.stellar.org/).

The binary reads an FBAS in [stellarbeat](https://stellarbeat.io/)'s JSON format and

- ranks the nodes using the [fbas_rewards_distributor](https://gitlab.informatik.hu-berlin.de/ti/theses/student-content/ndolo-charmaine-ma/fbas-reward-distributor)
- writes 2 files containing commonly used graph encodings, i.e.
    - a nodes list with weights (using one of the algorithms implemented in the above tool) for each node and
    - an adjacency list

All nodes with 'unsatisfiable' quorum sets are not included in the output and nodes marked as inactive can optionally be excluded from the output.

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
mobilecoin_nodes_2021-10-22_power_index_enum_adjacency_list.csv

```
