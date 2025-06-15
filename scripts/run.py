from config.global_config import GlobalConfig
from metadata_extraction.c_metadata import extract_c_metadata_from_project
from metadata_extraction.rust_metadata import c_metadata_to_rust_metadata
from llm.client import GenerationClient
from cache.cache import ProjectCache
from code_optim.code_gen import code_generation, code_verification
from code_optim.code_optim import code_optimization
from code_optim.predefined_agents import get_implicit_casting_removal_agent, get_as_bool_removal_agent, get_struct_index_advancement_agent, get_fix_mismatched_delim_agent, get_llm_repair_agent, get_definition_replace_agent



if __name__ == "__main__":
    config = GlobalConfig()
    config.project_name = "avl"
    extract_c_metadata_from_project(config)
    rust_metadata = c_metadata_to_rust_metadata(config)
    cache_0 = ProjectCache(config,cache_dir="0_cache")
    client = GenerationClient(config)
    code_generation(config, rust_metadata, cache_0, client)
    new_rust_metadata = c_metadata_to_rust_metadata(config)
    report_0 = code_verification(config, new_rust_metadata, cache_0, client)
    with open("report_0.json", "w") as f:
        import json
        json.dump(report_0, f, indent=4)
    
    cache_1 = ProjectCache(config, cache_dir="1_fmd_cache")
    new_rust_metadata = c_metadata_to_rust_metadata(config)
    IMPLICIT_CASTING_REMOVAL_AGENT = get_implicit_casting_removal_agent(config, new_rust_metadata)
    AS_BOOL_REMOVAL_AGENT = get_as_bool_removal_agent(config, new_rust_metadata)
    STRUCT_INDEX_ADVANCEMENT_AGENT = get_struct_index_advancement_agent(config, new_rust_metadata)
    FIX_MISMATCHED_DELIM_AGENT = get_fix_mismatched_delim_agent(config, new_rust_metadata, client)
    LLM_REPAIR_AGENT = get_llm_repair_agent(config, new_rust_metadata, client)
    DEFINITION_REPLACE_AGENT = get_definition_replace_agent(config, new_rust_metadata)
    report_1 = code_optimization(config, new_rust_metadata, cache_0, cache_1, client,
        optimizations={
            "macro": [FIX_MISMATCHED_DELIM_AGENT],
            "macro_function": [FIX_MISMATCHED_DELIM_AGENT],
            "definition": [FIX_MISMATCHED_DELIM_AGENT],
            "function": [FIX_MISMATCHED_DELIM_AGENT],
        })
    with open("report_1.json", "w") as f:
        import json
        json.dump(report_1, f, indent=4)

    cache_2 = ProjectCache(config, cache_dir="2_rule_cache")
    new_rust_metadata = c_metadata_to_rust_metadata(config)
    report_2 = code_optimization(config, new_rust_metadata, cache_1, cache_2, client,
        optimizations={
            "definition": [DEFINITION_REPLACE_AGENT],
            "function": [
                IMPLICIT_CASTING_REMOVAL_AGENT,
                AS_BOOL_REMOVAL_AGENT,
                STRUCT_INDEX_ADVANCEMENT_AGENT,
            ],
        })
    with open("report_2.json", "w") as f:
        import json
        json.dump(report_2, f, indent=4)
    