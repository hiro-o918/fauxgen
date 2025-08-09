use clap::{Parser, Subcommand};
use fauxgen_core::generator::write_factory_codes;
use log::{error, info};
use std::path::PathBuf;

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "fauxgen")]
#[command(about = "Python code generator to construct fake data for testing", long_about = None)]
#[command(version = env!("CARGO_PKG_VERSION"))]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate factory codes from a directory of Python files
    Gen {
        /// The root directory containing Python files to process
        #[arg(long, required = true)]
        module_dir: PathBuf,
        /// The output directory where generated factory codes will be saved
        #[arg(long)]
        output_dir: Option<PathBuf>,
    },
}

fn main() {
    let args = Cli::parse();
    env_logger::init();

    match args.command {
        Commands::Gen {
            module_dir,
            output_dir,
        } => {
            if !module_dir.exists() {
                error!("module dir does not exist: {}", module_dir.display());
                std::process::exit(1);
            }
            let output_dir = output_dir.unwrap_or_else(|| {
                let testing_path = module_dir.join("testing");
                // Create the testing directory if it doesn't exist
                if !testing_path.exists() {
                    std::fs::create_dir_all(&testing_path).unwrap_or_else(|_| {
                        error!(
                            "Failed to create testing directory: {}",
                            testing_path.display()
                        );
                        std::process::exit(1);
                    });
                }
                let init_path = testing_path.join("__init__.py");
                if !init_path.exists() {
                    std::fs::write(&init_path, "").unwrap_or_else(|_| {
                        error!("Failed to create __init__.py file: {}", init_path.display());
                        std::process::exit(1);
                    });
                }

                testing_path.join("fauxgen")
            });
            if let Err(e) = write_factory_codes(&module_dir, &output_dir) {
                error!("Error generating factory codes: {}", e);
                std::process::exit(1);
            }
            info!(
                "Factory codes generated successfully in {}",
                output_dir.display()
            );
        }
    }
}
