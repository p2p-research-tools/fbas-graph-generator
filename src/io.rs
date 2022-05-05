use fbas_analyzer::Fbas;
use std::fs::{self, File};
use std::io::{prelude::*, LineWriter};
use std::path::{Path, PathBuf};

pub(crate) fn create_output_dir(path: Option<&PathBuf>) -> Option<String> {
    let path_to_dir = if let Some(dir) = path {
        dir.as_path().display().to_string()
    } else {
        String::from("graphs")
    };
    if fs::create_dir_all(path_to_dir.clone()).is_ok() {
        Some(path_to_dir)
    } else {
        eprintln!("Error creating output directory..\nWill not create output files.");
        None
    }
}

/// Writes a node list to a text file in the format of
/// a header line of the format "Id,Label, weight" followed by the nodes in the FBAS
pub(crate) fn write_nodelist_to_file(
    output_dir: Option<String>,
    filename: String,
    node_list: Vec<String>,
) {
    if let Some(path_to_dir) = output_dir {
        let file_name = format!("{}/{}{}{}", path_to_dir, filename, "_nodelist", ".csv");
        let file = File::create(file_name.clone()).expect("Error creating file");
        let mut file = LineWriter::new(file);
        let header = "Id,Label,weight\n".as_bytes();
        file.write_all(header).unwrap();
        for line in node_list {
            file.write_all(line.as_bytes()).unwrap();
        }
        println!("Written node list to {}", file_name);
    };
}

/// Writes an adjaceny list to the text for all the nodes in the FBAS on a separate line
pub(crate) fn write_edgelist_to_file(
    output_dir: Option<String>,
    filename: String,
    adj_list: Vec<String>,
) {
    if let Some(path_to_dir) = output_dir {
        let file_name = format!(
            "{}/{}{}{}",
            path_to_dir, filename, "_adjacency_list", ".csv"
        );
        let file = File::create(file_name.clone()).expect("Error creating file");
        let mut file = LineWriter::new(file);
        for edges in adj_list {
            let line = format!("{}\n", edges);
            file.write_all(line.as_bytes()).unwrap();
        }
        println!("Written adjacency list to {}", file_name);
    };
}

pub(crate) fn load_fbas(nodes_path: &Path, ignore_inactive_nodes: bool) -> Fbas {
    eprintln!("Reading FBAS JSON from file...");
    let mut fbas = Fbas::from_json_file(nodes_path);
    if ignore_inactive_nodes {
        let inactive_nodes =
            fbas_analyzer::FilteredNodes::from_json_file(nodes_path, |v| v["active"] == false);
        fbas = fbas.without_nodes_pretty(&inactive_nodes.into_pretty_vec());
    }
    let unsatisfiable_nodes: Vec<usize> = fbas.unsatisfiable_nodes().into_iter().collect();
    fbas = fbas.without_nodes(&unsatisfiable_nodes);
    eprintln!("Processing FBAS with {} nodes.", fbas.number_of_nodes());
    fbas
}
