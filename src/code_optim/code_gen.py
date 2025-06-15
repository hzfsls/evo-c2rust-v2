from config.global_config import GlobalConfig
from llm.client import GenerationClient
from llm.generation import update_codes
from cache.cache import ProjectCache
from entity.metadata import RustProjectMetadata
from entity.project import RustProject
from tqdm import tqdm

import os

def code_generation(
    config: GlobalConfig,
    metadata: RustProjectMetadata,
    cache: ProjectCache,
    client: GenerationClient,
):
    project_name = config.project_name
    for typ in ["macro", "macro_function", "definition", "dummy_function", "function"]:
        # single_type_code_generation(config, metadata, typ, cache=cache, client=client)
        print(f"Project {project_name}: Start {typ} code generation.")
        codes = metadata.get_all(typ) if typ != "dummy_function" else metadata.get_all("function")
        update_codes(client, typ, codes, cache, multi_process=True, threads_num=10)

def code_verification(
    config: GlobalConfig,
    metadata: RustProjectMetadata,
    cache: ProjectCache,
    client: GenerationClient,
):  
    report = {}
    for typ in ["macro", "macro_function", "definition", "dummy_function", "function"]:
        typ_report = {
            "all_cnt": 0,
            "passed_cnt": 0,
            "pass_rate": 0.0,
            "messages": [],
        }
        print(f"Project {config.project_name}: Start {typ} compilation verification.")
        codes = metadata.get_all(typ) if typ != "dummy_function" else metadata.get_all("function")
        # update_codes(client, typ, codes, cache, multi_process=True, threads_num=10)
        all_cnt = len(codes)
        failed_cnt = 0
        for c in tqdm(codes):            
            placeholder_code = c.rust_code # The placeholder code to use if compilation fails
            c.rust_code = cache.get(typ, c.c_code)
            curr_cache_path = os.path.join(cache.caches[typ].path, cache.caches[typ].cache_index[c.c_code], "result.rs")
            proj = RustProject(config.project_name, metadata, config.created_project_dir, config.template_project_dir)
            success, error_msg = proj.build_project()
            if not success:                
                # print(f"Compilation failed for {typ} at {curr_cache_path}")
                failed_cnt += 1
                failed_message = {
                    "c_code": c.c_code,
                    "rust_code": c.rust_code,
                    "error_msg": error_msg,
                }
                c.rust_code = placeholder_code
                typ_report["messages"].append(failed_message)
        typ_report["all_cnt"] = all_cnt
        typ_report["passed_cnt"] = all_cnt - failed_cnt
        if all_cnt > 0:
            typ_report["pass_rate"] = (all_cnt - failed_cnt) / all_cnt
        else:
            typ_report["pass_rate"] = 100.0
        print(f"{typ} compilation verification completed. Passed: {typ_report['passed_cnt']}/{typ_report['all_cnt']} ({typ_report['pass_rate'] * 100:.2f}%)")
        report[typ] = typ_report
    return report