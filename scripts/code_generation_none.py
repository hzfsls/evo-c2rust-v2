from config.global_config import GlobalConfig
from metadata_extraction.c_metadata import extract_c_metadata_from_project
from metadata_extraction.rust_metadata import c_metadata_to_rust_metadata
from llm.client import GenerationClient
from cache.cache import ProjectCache
from code_optim.code_gen import code_generation

import argparse

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Run code verification on a project.")
    parser.add_argument("--project_name", type=str, required=True, help="Name of the project to run code verification on.")
    args = parser.parse_args()
    
    config = GlobalConfig()
    config.project_name = args.project_name
    config.definition_prompt = "Translate the C code to Rust code."
    config.macro_prompt = "Translate the C macro to Rust `macro_rules`. Add a line `pub(crate) use {{MACRO_NAME}};` to export the macro."
    config.macro_function_prompt = "Translate the C macro to Rust `macro_rules`. Add a line `pub(crate) use {{MACRO_NAME}};` to export the macro."
    config.function_prompt = "Translate the C Function to Rust Function."
    
    rust_metadata = c_metadata_to_rust_metadata(config)
    cache = ProjectCache(config, cache_dir="cache_non")
    client = GenerationClient(config)
    code_generation(config, rust_metadata, cache, client, multi_process=True, threads_num=5)