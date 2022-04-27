use fbas_analyzer::Fbas;
use fbas_reward_distributor::NodeRanking;

/// Gets an FBAS and returns a list or neighbours for every node
pub(crate) fn generate_adjacency_list(fbas: &Fbas) -> Vec<String> {
    let mut adj_list: Vec<Vec<String>> = Vec::default();
    for node in fbas.all_nodes().into_iter() {
        let own_list = vec![node.to_string()];
        adj_list.push(own_list);
    }
    for node in fbas.all_nodes().into_iter() {
        // this node has an edge to these nodes
        let contained_nodes = fbas.get_quorum_set(node).unwrap().contained_nodes();
        for target in contained_nodes.into_iter() {
            adj_list[target].push(node.to_string());
        }
        adj_list[node].dedup();
    }

    adj_list.iter().map(|nodelist| nodelist.join(" ")).collect()
}

/// Gets a vec of tupels of the type (NodeId, PublicKey, Score) and returns them as strings in the
/// same order, i.e. Vec<"Id, PK, score">
pub fn generate_node_list_with_weight(rankings: &[NodeRanking]) -> Vec<String> {
    let mut nodelist: Vec<String> = Vec::default();
    for node in rankings {
        let line = format!("{},{},{}\n", node.0, node.1, node.2);
        nodelist.push(line)
    }
    nodelist
}
