# fbas_trust_graph_generator

Generate weighted trust graphs for FBASs.

The binary takes an FBAS as an input

    1. ranks the nodes using the [fbas_rewards_distributor]()
    2. returns 2 files containing commonly used graph encodings, i.e. 1) a nodes list with weights for each node and 2) an adjacency matrix
