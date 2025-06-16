from config.global_config import GlobalConfig
from metadata_extraction.c_metadata import extract_c_metadata_from_project
from metadata_extraction.rust_metadata import c_metadata_to_rust_metadata
from llm.client import GenerationClient
from cache.cache import ProjectCache
from code_optim.code_gen import code_generation
from entity.project import RustProject

import argparse

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Run code verification on a project.")
    parser.add_argument("--project_name", type=str, required=True, help="Name of the project to run code verification on.")
    args = parser.parse_args()
    
    config = GlobalConfig()
    config.project_name = args.project_name
    extract_c_metadata_from_project(config)
    rust_metadata = c_metadata_to_rust_metadata(config)
    cache = ProjectCache(config, cache_dir="cache_3")
    client = GenerationClient(config)
    code_generation(config, rust_metadata, cache, client, multi_process=True, threads_num=5)
    # generation final project with rust_metadata
    project = RustProject(name=config.project_name, metadata=rust_metadata, parent_dir=config.final_project_dir, template_project_dir=config.template_project_dir, no_timestamp=True)
    
