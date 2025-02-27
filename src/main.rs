use std::{
    fs,
    io::{BufRead, BufReader, Write},
};

use clap::Parser;

use tcn::*;

fn skeleton_classes(
    nontroplanar_dir: String,
    genus: usize,
    out: String,
    tfile: String,
) -> Result<(), String> {
    // enusre we have correct genus as input
    assert!(genus > 3 && genus < 9);

    let filename = std::path::Path::new(&tfile)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();
    let file = match fs::File::open(tfile.clone()) {
        Ok(f) => f,
        Err(_) => return Err(format!("failed to open file {tfile}")),
    };
    let fbuf = BufReader::new(file);

    println!("Parsing triangulations and flips from {}", tfile);
    let (mut subdivisions, flips, mut subdivision_idxs) = utils::parse_input(
        fbuf.lines()
            .map(|l| l.expect(&format!("Could not parse line from {tfile}")))
            .collect(),
    );

    let nodes = genus - 3;

    for node in 1..=nodes {
        let genus_str = format!("genus{}", genus - node);
        let nontroplanar_file = format!("{}/{genus_str}.txt", nontroplanar_dir);
        println!("Computing genus {} graphs of {} triangulations with {} node(s)", genus - node, tfile, node);

	let out_dir = format!("./{out}/{genus_str}");
        match fs::DirBuilder::new().recursive(true).create(&out_dir) {
            Ok(_) => (),
            Err(_) => return Err(format!("Failed out create dir {out}")),
        }

        let mut out_f = match fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(format!("{out}/{genus_str}/from_{filename}"))
        {
            Ok(f) => f,
            Err(e) => {
                return Err(format!(
                    "Failed out create file ./{out}/{genus_str}/from_{filename} -- {e}"
                ))
            }
        };

        (subdivisions, subdivision_idxs) =
            crate::utils::apply_flips(&subdivisions, &flips, &subdivision_idxs, subdivisions.len());

        let mut buckets: std::collections::HashMap<String, Vec<graph::Graph>> =
            std::collections::HashMap::new();

        let filter = match crate::utils::parse_nonplanar_hashes(&nontroplanar_file) {
            Some(f) => f,
            None => return Err(format!("Could not read file {}", nontroplanar_file)),
        };

        for subd in &subdivisions {
            let mut skeletonized_graph = graph::Graph::new_skeleton(subd);
            let key = skeletonized_graph.categorizing_hash();

            let insert = filter.contains(&key);
            if insert {
                if let Some(v) = buckets.get_mut(&key) {
                    if v.iter().all(|g| !g.is_isomorphic(&skeletonized_graph)) {
                        v.push(skeletonized_graph);
                    }
                } else {
                    buckets.insert(key, vec![skeletonized_graph]);
                };
            }
        }

        // write all graphs up to isomorphism grouped by (Loops:Bridges:Bi-Edges:Sprawling)
        for (key, graphs) in buckets.iter() {
            match out_f.write_fmt(format_args!("{}\n", key)) {
                Ok(_) => (),
                Err(_) => {
                    return Err(format!(
                        "Failed to write key {key} to file {out}/{genus_str}!"
                    ))
                }
            };
            for graph in graphs {
                match out_f.write_fmt(format_args!("{}\n", graph)) {
                    Ok(_) => (),
                    Err(_) => {
                        return Err(format!(
                            "Failed to write graph {graph} to file {out}/{genus_str}!"
                        ))
                    }
                };
            }
            match out_f.write(b"\n") {
                Ok(_) => (),
                Err(_) => return Err(format!("Failed to write to file {out}/{genus_str}!")),
            };
        }
    }

    Ok(())
}

/// Program to compute trivalent graphs dual to nodal subdivisions of Newton polygons
/// expecting topcom triangulations and flips in command line
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Directory containing nonplanar graph classes for genus..=genus + 3
    #[arg(short, long)]
    nontroplanar_dir: String,

    /// Interior points of lattice polygon computing on
    /// will automatically compute up to nodes = genus - 3
    #[arg(short, long)]
    genus: usize,

    /// Directory to write file out to
    #[arg(short, long)]
    out: String,

    /// Topcom file with triangulations and flips
    #[arg(short, long)]
    tfile: String,
}

fn main() {
    let args = Args::parse();
    match skeleton_classes(args.nontroplanar_dir, args.genus, args.out, args.tfile) {
        Ok(_) => (),
        Err(e) => panic!("{e}"),
    };
}
