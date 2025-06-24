from config.global_config import GlobalConfig
from metadata_extraction.rust_metadata import c_metadata_to_rust_metadata
from cache.cache import ProjectCache
from code_optim.code_gen import blankfill_compilation_verification

import argparse

if __name__ == "__main__":
    final_report = {}
    projects = ["avl", "bzp", "cmptlz", "rapidlz", "md5", "sha256"]
    method_names = ["llm-zero-shot-ds", "evo-c2rust-v1-ds", "evo-c2rust-v2-ds-rule-fix", "evo-c2rust-v2-ds-llm-repair",  "evo-c2rust-v2-qwen-rule-fix"]
    for method_name in method_names:
        method_report = {}
        all_proj_total_cnt = 0
        all_proj_passed_cnt = 0
        all_proj_function_cnt = 0
        all_proj_function_passed_cnt = 0
        for project_name in projects:          
            config = GlobalConfig()
            config.project_dir = "./data/fill/project"
            config.c_metadata_dir = "./data/fill/c_metadata"
            config.rust_metadata_dir = "./data/fill/rust_metadata"
            config.project_name = project_name
            rust_metadata = c_metadata_to_rust_metadata(config)
            gold_cache = ProjectCache(config, cache_dir="./data/fill/cache/gold")
            cache = ProjectCache(config, cache_dir=f"./data/fill/cache/{method_name}")
            report = blankfill_compilation_verification(config, rust_metadata, gold_cache, cache)
            items = ["macro", "macro_function", "definition", "function"]
            calcs = ["all_cnt", "passed_cnt", "pass_rate"]
            report_1 = {item: {calc: report[item][calc] for calc in calcs} for item in items}
            total_all_cnt = sum(report[item]["all_cnt"] for item in items)
            total_passed_cnt = sum(report[item]["passed_cnt"] for item in items)
            total_passed_rate = total_passed_cnt / total_all_cnt
            all_proj_total_cnt += total_all_cnt
            all_proj_passed_cnt += total_passed_cnt
            all_proj_function_cnt += report["function"]["all_cnt"]
            all_proj_function_passed_cnt += report["function"]["passed_cnt"]
            report_1["total"] = {"all_cnt": total_all_cnt, "passed_cnt": total_passed_cnt, "pass_rate": total_passed_rate}
            print(report_1)
            method_report[project_name] = report_1
        all_proj_total_pass_rate = all_proj_passed_cnt / all_proj_total_cnt
        all_proj_function_pass_rate = all_proj_function_passed_cnt / all_proj_function_cnt
        method_report["all"] = {"total" : {"all_cnt": all_proj_total_cnt, "passed_cnt": all_proj_passed_cnt, "pass_rate": all_proj_total_pass_rate}, "function": {"all_cnt": all_proj_function_cnt, "passed_cnt": all_proj_function_passed_cnt, "pass_rate": all_proj_function_pass_rate},
        }
        final_report[method_name] = method_report
    with open("report.json", "w") as f:
        import json
        json.dump(final_report, f, indent=4)