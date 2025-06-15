from config.global_config import GlobalConfig
from llm.client import GenerationClient
from llm.generation import update_codes
from cache.cache import ProjectCache

def code_generation(
    config: GlobalConfig,
    metadata: RustProjectMetadata,
    cache: ProjectCache,
    client: GenerationClient,
):
    for typ in ["macro", "macro_function", "definition", "dummy_function", "function"]:
        # single_type_code_generation(config, metadata, typ, cache=cache, client=client)
        print(f"Start {typ} filling and compilation verification.")
        codes = metadata.get_all(typ) if typ != "dummy_function" else metadata.get_all("function")
        update_codes(client, typ, codes, cache)


# def single_type_code_generation(
#     config: GlobalConfig,
#     metadata: RustProjectMetadata
#     typ: str,
#     optimizations=[],
#     cache: ProjectCache,
# ):
#     get_name = {
#         "macro": "macro",
#         "macro_function": "macro_function",
#         "definition": "definition",
#         "dummy_function": "function",
#         "function": "function",
#     }
#     print(f"Start {typ} filling and compilation verification.")
#     codes = metadata.get_all(get_name[typ])
#     output = []
#     update_codes(client, typ, codes, cache)
#     # for idx, c in enumerate(codes):
#     #     if c.c_code in cache.caches[typ].cache:
#     #         c.rust_code = cache.caches[typ].get(c.c_code)
#     #         continue
#     #     c.rust_code = get_llm_gen_result(client, c.c_code, client.config.prompts[typ])
#     #     if cache is not None:
#     #         cache.caches[typ].update(c.c_code, c.rust_code)
#     #     output.append(c)



#     # if not fast:
#     #     for idx, c in enumerate(tqdm(codes)):
#     #         if allow_error:
#     #             original_code = c.rust_code
#     #         update_codes(type, client, prompts, [c], caches)
#     #         curr_cache_path = os.path.join(
#     #             cache.path, cache.cache_index[c.c_code], "result.rs"
#     #         )
#     #         proj = RustProject(proj_name, metadata, created_project_dir, template_project_dir)
#     #         success, error_msg = proj.build_project()
#     #         original_error_msg = error_msg
#     #         if not success:
#     #             curr_code, status, error_msg = c.rust_code, False, error_msg
#     #             if idx > 97:
#     #                 for o in optimizations:
#     #                     curr_code, status, error_msg = o.try_optimize(c)
#     #             if not status:
#     #                 if not allow_error:
#     #                     raise RustProjectCompilationFailedError(
#     #                         error_msg + "\n" + "Error at:" + curr_cache_path
#     #                     )
#     #                 else:
#     #                     tuple = {
#     #                         "c_code": c.c_code,
#     #                         "rust_code": curr_code,
#     #                         "error_msg": error_msg,
#     #                     }
#     #                     output.append(tuple)
#     #                     print(error_msg + "\n" + "Error at:" + curr_cache_path)
#     #                     cache.update(c.c_code, curr_code)
#     #                     c.rust_code = original_code
#     #                     # c.rust_code = curr_code
                        
#     #             else:
#     #                 c.rust_code = curr_code
#     #                 cache.update(c.c_code, c.rust_code)
#     # else:
#     #     if fast_end_idx == -1 or fast_end_idx >= len(codes):
#     #         update_codes(type, client, prompts, codes, caches)
#     #         proj = RustProject(proj_name, metadata, created_project_dir, template_project_dir)
#     #         success, error_msg = proj.build_project()
#     #         if not success:
#     #             if not allow_error:
#     #                 raise RustProjectCompilationFailedError(error_msg)
#     #             else:
#     #                 pass
#     #                 # print(error_msg)
#     #     else:
#     #         fast_filling_codes = codes[:fast_end_idx]
#     #         update_codes(type, client, prompts, fast_filling_codes, caches)
#     #         proj = RustProject(proj_name, metadata, created_project_dir, template_project_dir)
#     #         success, error_msg = proj.build_project()
#     #         if not success:
#     #             if not allow_error:
#     #                 raise RustProjectCompilationFailedError(error_msg)
#     #             else:
#     #                 pass
#     #                 # print(error_msg)
#     #         remaining_codes = codes[fast_end_idx:]
#     #         for c in tqdm(remaining_codes):
#     #             update_codes(type, client, prompts, [c], caches)                
#     #             curr_cache_path = os.path.join(
#     #                 cache.path, cache.cache_index[c.c_code], "result.rs"
#     #             )
#     #             proj = RustProject(proj_name, metadata, created_project_dir, template_project_dir)
#     #             success, error_msg = proj.build_project()
#     #             if not success:
#     #                 for o in optimizations:
#     #                     curr_code, status, error_msg = o.try_optimize(c)
#     #                 if not status:
#     #                     if not allow_error:
#     #                         raise RustProjectCompilationFailedError(
#     #                             error_msg + "\n" + "Error at:" + curr_cache_path
#     #                         )
#     #                     else:
#     #                         pass
#     #                         # print(error_msg + "\n" + "Error at:" + curr_cache_path)
#     #                 else:
#     #                     c.rust_code = curr_code
#     #                     cache.update(c.c_code, c.rust_code)
#     # return output