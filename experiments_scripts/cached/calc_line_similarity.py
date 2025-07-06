from config.global_config import GlobalConfig
from metadata_extraction.rust_metadata import c_metadata_to_rust_metadata
from cache.cache import ProjectCache
from code_optim.code_gen import blankfill_test
from entity.project import RustProject
import argparse
import os

import difflib

def count_common_lines_diff(file1_path, file2_path):
    with open(file1_path, 'r') as file1, open(file2_path, 'r') as file2:
        file1_lines = [l.strip() for l in file1.readlines() if l.strip() != ""]
        file2_lines = [l.strip() for l in file2.readlines() if l.strip() != ""]

        counter_1 = {}
        counter_2 = {}

        for l in file1_lines:
            counter_1[l] = counter_1.get(l, 0) + 1
        for l in file2_lines:
            counter_2[l] = counter_2.get(l, 0) + 1

        counter_s = {}
        for l in counter_1.keys():
            counter_s[l] = min(counter_1.get(l, 0), counter_2.get(l, 0))
        for l in counter_2.keys():
            counter_s[l] = min(counter_1.get(l, 0), counter_2.get(l, 0))
        
        accept_lines = sum(counter_s.values())
        total_lines_generated = sum(counter_1.values())
        total_lines_gold = sum(counter_2.values())

        recall = accept_lines / total_lines_gold
        precision = accept_lines / total_lines_generated
        return {
            "total_gold_lines": total_lines_gold,
            "total_generated_lines": total_lines_generated,
            "accept_lines": accept_lines,
            "recall": recall,
            "precision": precision
        }

if __name__ == "__main__":
    final_report = {}
    projects = ["avl", "bzp", "cmptlz", "rapidlz", "md5", "sha256"]
    method_name = "evo-c2rust-v2-ds-human-repair"

    final_report = {}

    total = {
        "total_gold_lines": 0,
        "total_generated_lines": 0,
        "accept_lines": 0,
        "recall": None,
        "precision": None
    }

    for project_name in projects:          
        config = GlobalConfig()
        config.project_dir = "./data/fill-calc-line-accept/project"
        config.c_metadata_dir = "./data/fill-calc-line-accept/c_metadata"
        config.rust_metadata_dir = "./data/fill-calc-line-accept/rust_metadata"
        config.template_project_dir = "./data/project_template/safelevel-0-test"
        config.project_name = project_name
        rust_metadata = c_metadata_to_rust_metadata(config)
        cache = ProjectCache(config, cache_dir=f"./data/fill-calc-line-accept/cache/{method_name}")
        with open(os.path.join("./data/fill-calc-line-accept/testcases", project_name, "mod.rs"), "r") as f:
            mod_rs_content = f.read() 
        rust_metadata.set_mod_rs(mod_rs_content)
        # verify gold answer
        for typ in ["macro", "macro_function", "definition", "function"]:
            codes = rust_metadata.get_all(typ)
            # fill codes with gold cache
            for c in codes:
                c.rust_code = cache.get(typ, c.c_code)
        proj = RustProject(config.project_name, rust_metadata, config.created_project_dir, config.template_project_dir)
        success, error_msg = proj.test_project()
        assert success, f"{project_name} gold answer failed, {error_msg}"

        generated_path = f"./data/fill-calc-line-accept/final_files/generated/{project_name}_c.rs"
        gold_path = f"./data/fill-calc-line-accept/final_files/gold/{project_name}_c.rs"

        result = count_common_lines_diff(generated_path, gold_path)
        final_report[project_name] = result

        total["accept_lines"] += result["accept_lines"]
        total["total_gold_lines"] += result["total_gold_lines"]
        total["total_generated_lines"] += result["total_generated_lines"]
    
    total["precision"] = total["accept_lines"] / total["total_generated_lines"]
    total["recall"] = total["accept_lines"] / total["total_gold_lines"]
    final_report["total"] = total

    with open("report-cached-line-similarity.json", "w") as f:
        import json
        json.dump(final_report, f, indent=4)
