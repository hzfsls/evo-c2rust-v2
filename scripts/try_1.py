from config.global_config import GlobalConfig
from metadata_extraction.c_metadata import extract_c_metadata_from_project
from metadata_extraction.rust_metadata import c_metadata_to_rust_metadata
from llm.client import GenerationClient
from cache.cache import ProjectCache
from code_optim.code_gen import blankfill_compilation_verification
from entity.project import RustProject
import os
import argparse
import json

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Run code verification on a project.")
    parser.add_argument("--project_name", type=str, required=True, help="Name of the project to run code verification on.")
    args = parser.parse_args()
    
    config = GlobalConfig()
    config.project_name = args.project_name
    rust_metadata = c_metadata_to_rust_metadata(config)
    gold_cache = ProjectCache(config, cache_dir="cache_gold")
    cache = ProjectCache(config, cache_dir="cache_2")
    client = GenerationClient(config)
    report = blankfill_compilation_verification(config, rust_metadata, gold_cache, cache, client)
    with open("report.json", "w") as f:
        json.dump(report, f, indent=4)
