from config.global_config import GlobalConfig
from metadata_extraction.rust_metadata import c_metadata_to_rust_metadata
from cache.cache import ProjectCache
from code_optim.code_gen import blankfill_test

import argparse
import os

if __name__ == "__main__":
    final_report = {}
    projects = ["avl", "bzp", "cmptlz", "rapidlz", "md5", "sha256"]
    method_names = ["evo-c2rust-v2-ds-llm-repair",  "evo-c2rust-v2-qwen-rule-fix"]
    for method_name in method_names:
        method_report = {}
        all_proj_total_cnt = 0
        all_proj_passed_cnt = 0
        all_proj_function_cnt = 0
        all_proj_function_passed_cnt = 0
        for project_name in projects:          
            config = GlobalConfig()
            config.project_dir = "./data/fill-with-cache/project"
            config.c_metadata_dir = "./data/fill-with-cache/c_metadata"
            config.rust_metadata_dir = "./data/fill-with-cache/rust_metadata"
            config.template_project_dir = "./data/project_template/safelevel-0-test"
            config.project_name = project_name
            rust_metadata = c_metadata_to_rust_metadata(config)
            gold_cache = ProjectCache(config, cache_dir="./data/fill-with-cache/gold_cache")
            cache = ProjectCache(config, cache_dir=f"./data/fill-with-cache/cache/{method_name}")
            with open(os.path.join("./data/fill-with-cache/testcases", project_name, "mod.rs"), "r") as f:
                mod_rs_content = f.read() 
            rust_metadata.set_mod_rs(mod_rs_content)
            report = blankfill_test(config, rust_metadata, gold_cache, cache)
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
        
    with open("report-cached-semantic-accuracy.json", "w") as f:
        import json
        json.dump(final_report, f, indent=4)