from c_metadata.c_metadata import extract_c_metadata_from_project
from c_metadata.rust_project_creation import c_metadata_to_rust_metadata

if __name__ == "__main__":
    config = {
        "c_metadata_dir": "./config/c_metadata",
        "rust_metadata_dir": "./config/rust_metadata",
        "created_project_dir": "./config/created_project",
        "template_project_dir": "./config/template_project"
    }
    extract_c_metadata_from_project(proj_name, config)
    metadata = c_metadata_to_rust_metadata(proj_name, c_metadata_dir=c_metadata_dir, rust_metadata_dir=rust_metadata_dir, created_project_dir=created_project_dir, template_project_dir=template_project_dir)