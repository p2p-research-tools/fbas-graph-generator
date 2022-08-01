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
    overwrite: bool,
) -> Result<String, std::io::Error> {
    if let Some(path_to_dir) = output_dir {
        let file_name = format!("{}/{}{}{}", path_to_dir, filename, "_nodelist", ".csv");
        if !overwrite {
            assert!(
                !Path::new(&file_name).exists(),
                "File {} exists. Refusing to overwrite.\n Use --overwrite if this is intended.",
                file_name
            );
        }
        let file = File::create(file_name.clone()).expect("Error creating file");
        let mut file = LineWriter::new(file);
        let header = "Id,Label,weight\n".as_bytes();
        file.write_all(header).unwrap();
        for line in node_list {
            file.write_all(line.as_bytes()).unwrap();
        }
        println!("Written node list to {}", file_name);
        return Ok(file_name);
    };
    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "invalid directory path",
    ))
}

/// Writes an adjaceny matrix to the text for all the nodes in the FBAS on a separate line
pub(crate) fn write_adjacency_matrix_to_file(
    output_dir: Option<String>,
    filename: String,
    adj_list: Vec<String>,
    overwrite: bool,
) -> Result<String, std::io::Error> {
    if let Some(path_to_dir) = output_dir {
        let file_name = format!(
            "{}/{}{}{}",
            path_to_dir, filename, "_adjacency_matrix", ".csv"
        );
        if !overwrite {
            assert!(
                !Path::new(&file_name).exists(),
                "File {} exists. Refusing to overwrite.\n Use --overwrite if this is intended.",
                file_name
            );
        }
        let file = File::create(file_name.clone()).expect("Error creating file");
        let mut file = LineWriter::new(file);
        for node in adj_list {
            let line = format!("{}\n", node);
            file.write_all(line.as_bytes()).unwrap();
        }
        println!("Written adjacency list to {}", file_name);
        return Ok(file_name);
    };
    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "invalid directory path",
    ))
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
#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::*;
    use tempfile::TempDir;

    #[test]
    fn create_dir() {
        let tmp_dir = TempDir::new().expect("Error creating TempDir");
        let output_path = tmp_dir.path().to_path_buf();
        let expected = create_output_dir(Some(&output_path));
        let actual = Some(tmp_dir.path().display().to_string());
        assert_eq!(expected, actual);
    }

    #[test]
    fn write_edgelist() {
        let fbas = Fbas::from_json_file(Path::new("test_data/trivial.json"));
        let output_dir = TempDir::new().expect("Error creating TempDir");
        let overwrite = false;
        let file_name = String::from("edgelist-test-file");
        let adj_list = generate_adjacency_matrix(&fbas);
        let output_file_path = write_adjacency_matrix_to_file(
            Some(output_dir.path().display().to_string()),
            file_name.clone(),
            adj_list,
            overwrite,
        );
        assert!(output_file_path.is_ok());
    }

    #[test]
    fn write_nodelist() {
        let output_dir = TempDir::new().expect("Error creating TempDir");
        let overwrite = false;
        let file_name = String::from("nodelist-test-file");
        let rankings = vec![
            (0, "nodeA".to_string(), 0.0),
            (1, "nodeB".to_string(), 0.1),
            (2, "nodeC".to_string(), 0.2),
        ];
        let node_list = generate_node_list_with_weight(&rankings);
        let output_file_path = write_nodelist_to_file(
            Some(output_dir.path().display().to_string()),
            file_name.clone(),
            node_list,
            overwrite,
        );
        assert!(output_file_path.is_ok());
    }
}
