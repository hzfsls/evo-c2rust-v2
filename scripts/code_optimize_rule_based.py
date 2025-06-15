from config.global_config import GlobalConfig
from metadata_extraction.c_metadata import extract_c_metadata_from_project
from metadata_extraction.rust_metadata import c_metadata_to_rust_metadata
from llm.client import GenerationClient
from cache.cache import ProjectCache
from code_optim.code_gen import code_generation, code_verification
from code_optim.code_optim import code_optimization
from code_optim.predefined_agents import get_implicit_casting_removal_agent, get_as_bool_removal_agent, get_struct_index_advancement_agent, get_fix_mismatched_delim_agent, get_llm_repair_agent, get_definition_replace_agent

import argparse
import json

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Run code verification on a project.")
    parser.add_argument("--project_name", type=str, required=True, help="Name of the project to run code verification on.")
    args = parser.parse_args()
    
    config = GlobalConfig()
    config.project_name = args.project_name
    rust_metadata = c_metadata_to_rust_metadata(config)
    old_cache = ProjectCache(config, cache_dir="cache_1")
    new_cache = ProjectCache(config, cache_dir="cache_2")
    client = GenerationClient(config)

    IMPLICIT_CASTING_REMOVAL_AGENT = get_implicit_casting_removal_agent(config, rust_metadata)
    AS_BOOL_REMOVAL_AGENT = get_as_bool_removal_agent(config, rust_metadata)
    STRUCT_INDEX_ADVANCEMENT_AGENT = get_struct_index_advancement_agent(config, rust_metadata)
    DEFINITION_REPLACE_AGENT = get_definition_replace_agent(config, rust_metadata)

    report = code_optimization(config, rust_metadata, old_cache, new_cache, client,
        optimizations={
            "definition": [DEFINITION_REPLACE_AGENT],
            "function": [IMPLICIT_CASTING_REMOVAL_AGENT, AS_BOOL_REMOVAL_AGENT, STRUCT_INDEX_ADVANCEMENT_AGENT],
        })
    with open("report_2.json", "w") as f:
        import json
        json.dump(report, f, indent=4)