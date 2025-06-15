from config.global_config import GlobalConfig
from metadata_extraction.c_metadata import extract_c_metadata_from_project
from metadata_extraction.rust_project_creation import c_metadata_to_rust_metadata
if __name__ == "__main__":
    config = GlobalConfig()
    config.project_name = "avl"
    extract_c_metadata_from_project(config)
    rust_metadata = c_metadata_to_rust_metadata(config)