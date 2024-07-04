mod utils;

use std::fs::{self, OpenOptions};

use anyhow::{bail, Context};
use threemf::model::{
    Build, Component, Item, Metadata, Model, Object, ObjectData, Resources, Unit,
};

use clap::Parser;

/// Merges multiple STL files
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Name of a group of STLs in the final .3MF file.
    #[clap(long)]
    output: String,
    #[clap(long)]
    /// Output filename.
    name: Option<String>,
    /// List of STL files to merge.
    stl_files: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if cli.stl_files.is_empty() {
        bail!("No STL files provided.");
    }

    let mut meshes = Vec::with_capacity(cli.stl_files.len());

    for filename in cli.stl_files {
        println!("Merging file: {}", filename);

        let mesh = {
            let mut file = OpenOptions::new()
                .read(true)
                .open(&filename)
                .context("Read STL files")?;
            let stl = stl_io::read_stl(&mut file).context("Parse STL file")?;
            utils::stl_to_mesh(stl)
        };

        meshes.push((filename, mesh));
    }

    // Add basic metadata to the 3MF file.
    let metadata = vec![Metadata {
        name: "Application".to_string(),
        value: Some("stlto3mf".to_string()),
    }];

    // Convert the meshes into objects.
    let mut objects: Vec<_> = meshes
        .into_iter()
        .enumerate()
        .map(|(id, (name, mesh))| Object {
            id,
            partnumber: Some(name.clone()),
            name: Some(name),
            pid: None,
            object: ObjectData::Mesh(mesh),
        })
        .collect();

    // Find the highest object ID so we can assign a new one to the build object.
    let highest_object_id = objects.iter().map(|o| o.id).max().unwrap(); // SAFETY: We've checked there is at least single STL file loaded.
    let build_object_id = highest_object_id + 1;

    let components: Vec<_> = objects
        .iter()
        .map(|object| Component {
            objectid: object.id,
            transform: None,
        })
        .collect();

    let name = match cli.name {
        Some(name) => name,
        None => {
            // If no name is provided, use the names of the STL files as the name of the group.
            // This is a bit of a hack, but it's better than nothing.
            let mut names = Vec::new();
            for object in &objects {
                if let Some(name) = &object.name {
                    names.push(name.clone());
                }
            }
            names.join("+")
        }
    };

    objects.push(Object {
        id: build_object_id,
        partnumber: Some(name.clone()),
        name: Some(name),
        pid: None,
        object: ObjectData::Components {
            component: components,
        },
    });

    let model = Model {
        xmlns: "http://schemas.microsoft.com/3dmanufacturing/core/2015/02".to_string(),
        metadata,
        resources: Resources {
            object: objects,
            basematerials: None,
        },
        build: Build {
            item: vec![Item {
                objectid: build_object_id,
                transform: None,
                partnumber: None,
            }],
        },
        unit: Unit::Millimeter,
    };

    threemf::write::write(&mut fs::File::create(cli.output).unwrap(), model)
        .context("Write 3mf file")?;

    Ok(())
}
