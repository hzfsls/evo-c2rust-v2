from config.global_config import GlobalConfig
from metadata_extraction.c_metadata import extract_c_metadata_from_project
from metadata_extraction.rust_metadata import c_metadata_to_rust_metadata
# 将llm.client改为llm.client_hw
# 在其他脚本中使用内部大模型时，也需要做如此更改
from llm.client_hw import GenerationClient
from cache.cache import ProjectCache
from code_optim.code_gen import code_generation

import argparse

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Run code verification on a project.")
    parser.add_argument("--project_name", type=str, required=True, help="Name of the project to run code verification on.")
    args = parser.parse_args()
    
    config = GlobalConfig()
    config.project_dir = "./data/default/project"
    config.c_metadata_dir = "./data/default/c_metadata"
    config.rust_metadata_dir = "./data/default/rust_metadata"
    
    config.project_name = args.project_name
    
    # 更改你的api_key信息
    self.api_key = "sk-1234"
    self.base_url = "http://api.openai.rnd.huawei.com/v1"
    self.model_name = "qwen3-32b"
    client = GenerationClient(config)
    
    rust_metadata = c_metadata_to_rust_metadata(config)
    cache = ProjectCache(config, cache_dir="./data/default/cache/evo-c2rust-v2-qwen-gen-only")
    
    # 单线程
    code_generation(config, rust_metadata, cache, client, multi_process=False)