use fbas_analyzer::NodeId;
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
    about = "Generate the trust graph of an FBAS and output in common graph IO formats.",
    author = "Charmaine Ndolo"
)]
struct Opt {
    /// Path to directory where file should be saved.
    /// Defaults to "./graphs".
    #[structopt(short = "o", long = "output")]
    output: Option<PathBuf>,

    /// Overwrite file if it is present in the filesystem.
    /// Default behaviour is not to do this and abort a file is found with the same name.
    #[structopt(short, long)]
    overwrite: bool,

    /// Path to JSON file describing the FBAS in stellarbeat.org "nodes" format.
    /// Will use STDIN if omitted.
    nodes_path: PathBuf,

    /// Ranking algorithm to use.
    #[structopt(subcommand)]
    alg_config: Option<RankingAlgConfig>,

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
    PowerIndexEnum,
    /// Approximate Shapley values as a measure of nodes' importance in the FBAS. The number of
    /// samples to use must be passed if selected.
    /// The computation of minimal quorums can optionally be done before we start approximation.
    /// Useful, e.g. for timing measurements.
    PowerIndexApprox { s: usize },
}

fn main() {
    let args = Opt::from_args();
    let qi_check = !args.dont_check_for_qi;
    let alg = match args.alg_config {
        Some(RankingAlgConfig::NodeRank) => Some(RankingAlg::NodeRank),
        Some(RankingAlgConfig::PowerIndexEnum) => Some(RankingAlg::PowerIndexEnum(None)),
        Some(RankingAlgConfig::PowerIndexApprox { s }) => {
            Some(RankingAlg::PowerIndexApprox(s, None))
        }
        None => None,
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
        Some(RankingAlg::NodeRank) => String::from("node_rank"),
        Some(RankingAlg::PowerIndexEnum(_)) => String::from("power_index_enum"),
        Some(RankingAlg::PowerIndexApprox(_, _)) => String::from("power_index_approx"),
        None => String::from("unweighted"),
    };
    output_path = format!("{}_{}", output_path, alg_as_str);
    let fbas = io::load_fbas(&input_filename, args.ignore_inactive_nodes);
    let node_ids: Vec<NodeId> = (0..fbas.all_nodes().len()).collect();
    let mut rankings = graph::compute_influence(&node_ids, &fbas, alg.clone(), args.pks, qi_check);
    // normalise noderank scores
    if let Some(algo) = alg {
        if algo == RankingAlg::NodeRank {
            let node_rank_sum: Score = rankings.iter().map(|v| v.2 as Score).sum();
            for (_, node_ranking) in rankings.iter_mut().enumerate() {
                let normalised_ranking = node_ranking.2 / node_rank_sum;
                node_ranking.2 = f64::trunc(normalised_ranking * 1000.0) / 1000.0;
            }
        }
    }
    let adj_list = graph::generate_adjacency_matrix(&fbas);
    let node_list = graph::generate_node_list_with_weight(&rankings);
    let output_dir = io::create_output_dir(args.output.as_ref());
    if output_dir.is_some() {
        io::write_nodelist_to_file(
            output_dir.clone(),
            output_path.clone(),
            node_list,
            args.overwrite,
        )
        .expect("Encountered error while writing node list to file.");
        io::write_adjacency_matrix_to_file(output_dir, output_path, adj_list, args.overwrite)
            .expect("Encountered error while writing adjacency list to file.");
    } else {
        eprintln!("unable to write to specified output dir");
    }
}
