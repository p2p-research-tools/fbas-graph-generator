use fbas_analyzer::{Fbas, NodeId};
use fbas_reward_distributor::*;

use std::path::PathBuf;
use structopt::StructOpt;

mod graph;
mod io;

/// Rank nodes of an FBAS and write the results as a graph in a CSV.
/// Output files are names akin to the input with some additions such as the ranking algorithm used
/// and type of data stored in the file.
#[derive(Debug, StructOpt)]
#[structopt(
    name = "graph_generator",
    about = "Rank nodes of an FBAS and write the results as a graph in a CSV",
    author = "Charmaine Ndolo"
)]
struct Opt {
    /// Path to directory where file should be saved.
    /// Defaults to "
    #[structopt(short, long)]
    output: Option<PathBuf>,

    /// Path to JSON file describing the FBAS in stellarbeat.org "nodes" format.
    /// Will use STDIN if omitted.
    nodes_path: PathBuf,

    #[structopt(subcommand)]
    alg_config: RankingAlgConfig,

    /// Prior to any analysis, filter out all nodes marked as `"active" == false` in the input
    /// nodes JSON (the one at `nodes_path`).
    #[structopt(short = "i", long = "ignore-inactive-nodes")]
    ignore_inactive_nodes: bool,

    /// Do not assert that the FBAS has quorum intersection before proceeding with further computations.
    /// Default behaviour is to always check for QI.
    #[structopt(short = "nq", long = "no-quorum-intersection")]
    dont_check_for_qi: bool,

    /// Identify nodes by their public key where possible.
    /// Default is not to and an empty string is written in the "label" field.
    #[structopt(short = "p", long = "pretty")]
    pks: bool,
}

#[derive(Debug, StructOpt)]
enum RankingAlgConfig {
    /// Use NodeRank, an extension of PageRank, to measure nodes' weight in the FBAS
    NodeRank,
    /// Use Shapley-Shubik power indices to calculate nodes' importance in the FBAS. Not
    /// recommended for FBAS with many players because of time complexity
    ExactPowerIndex,
    /// Approximate Shapley values as a measure of nodes' importance in the FBAS. The number of
    /// samples to use must be passed if selected.
    /// The computation of minimal quorums can optionally be done before we start approximation.
    /// Useful, e.g. for timing measurements.
    ApproxPowerIndex { s: usize },
}

/// Rank nodes using either S-S Power Index or NodeRank and return a sorted list of nodes
fn compute_influence(
    node_ids: &[NodeId],
    fbas: &Fbas,
    alg: RankingAlg,
    use_pks: bool,
    qi_check: bool,
) -> Vec<NodeRanking> {
    let rankings = rank_nodes(fbas, alg, qi_check);
    create_node_ranking_report(node_ids, rankings, fbas, use_pks)
}

fn main() {
    let args = Opt::from_args();
    let qi_check = !args.dont_check_for_qi;
    let alg = match args.alg_config {
        RankingAlgConfig::NodeRank => RankingAlg::NodeRank,
        RankingAlgConfig::ExactPowerIndex => RankingAlg::ExactPowerIndex(None),
        RankingAlgConfig::ApproxPowerIndex { s } => RankingAlg::ApproxPowerIndex(s, None),
    };
    let input_filename = args.nodes_path;
    let mut output_path = match input_filename.file_stem() {
        Some(name) => match name.to_str() {
            Some(name) => name.to_string(),
            None => panic!("error occured while creating input file name"),
        },
        None => panic!("error occured while reading input file name"),
    };
    let alg_as_str = match alg {
        RankingAlg::NodeRank => String::from("node_rank"),
        RankingAlg::ExactPowerIndex(_) => String::from("exact_power_index"),
        RankingAlg::ApproxPowerIndex(_, _) => String::from("approx_power_index"),
    };
    output_path = format!("{}_{}", output_path, alg_as_str);
    let fbas = io::load_fbas(&input_filename, args.ignore_inactive_nodes);
    let node_ids: Vec<NodeId> = (0..fbas.all_nodes().len()).collect();
    let rankings = compute_influence(&node_ids, &fbas, alg, args.pks, qi_check);
    let adj_list = graph::generate_adjacency_list(&fbas);
    let node_list = graph::generate_node_list_with_weight(&rankings);
    let output_dir = io::create_output_dir(args.output.as_ref());
    if output_dir.is_some() {
        io::write_nodelist_to_file(output_dir.clone(), output_path.clone(), node_list);
        io::write_edgelist_to_file(output_dir, output_path, adj_list);
    } else {
        eprintln!("unable to write to specified output dir");
    }
}
