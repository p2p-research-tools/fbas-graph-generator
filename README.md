# fbas_graph_generator

Generate weighted trust graphs for FBASs like [Stellar](https://www.stellar.org/).

The binary takes an FBAS in [stellarbeat](https://stellarbeat.io/)'s JSON format

    1. ranks the nodes using the [fbas_rewards_distributor]()
    2. returns 2 files containing commonly used graph encodings, i.e. 1) a nodes list with weights for each node and 2) an adjacency matrix

All nodes with 'unsatisfiable' quorum sets are not included in the output and nodes marked as inactive can optionally be excluded from the output.

Run the following for usage instructions:

```
cargo run --release -- -h
```

## Example

The command

```
cargo run --release -- test_data/mobilecoin_nodes_2021-10-22.json -i exact-power-index
```

ranks the nodes in the FBAS and creates the following two files in the `graphs/` directory

```
mobilecoin_nodes_2021-10-22_exact_power_index_nodelist.csv
mobilecoin_nodes_2021-10-22_exact_power_index_adjacency_list.csv

```
