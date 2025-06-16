import configparser
import os

from config.prompt_config import definition_prompt, macro_prompt, macro_function_prompt, dummy_function_prompt, function_prompt, delim_repair_prompt, repair_prompt

# def read_config():
#     config = configparser.ConfigParser()
#     config.read(os.path.join(os.path.dirname(__file__), 'config.ini'))
#     return config

class GlobalConfig:
    def __init__(self):
        # config = read_config()
        self.project_name = None
        self.project_dir = "./data"
        self.created_project_dir = "./.tmp/created_project"
        self.c_metadata_dir = "./c_metadata"
        self.src_folders = ["include", "src"]
        self.macros = {
            "ALWAYS_INLINE": "inline",
            "ALWAYS_NO_INLINE": "",
            "STATIC": "static",
            "HIDDEN": "",
            "CMPTLZ_HIDDEN": "",
            "TARGET_ATTRIBUTE_AUTO": "",
            "RAPIDLZ_ALWAYS_INLINE": "inline",
            "CSTL_STATIC": "static",
            "DT_EXPORT": ""
        }
        self.replacements = {
            "__asm": "asm",
            "args...": "...",
            "##args": "__VA_ARGS__",
            "#if __cplusplus\nextern \"C\" {\n#endif": "extern \"C\" {\n",
            "#if __cplusplus\n}\n#endif": "}\n",
            "#if __cplusplus\n}\n\n#endif": "}\n"
        }
        self.rust_metadata_dir = "./rust_metadata"
        self.template_project_dir = "./project_template"

        self.definition_prompt = definition_prompt
        self.macro_prompt = macro_prompt
        self.macro_function_prompt = macro_function_prompt
        self.dummy_function_prompt = dummy_function_prompt
        self.function_prompt = function_prompt
        self.delim_repair_prompt = delim_repair_prompt
        self.repair_prompt = repair_prompt

        self.api_key = "sk-76da526dbd8b48c3954df9336a8a6592"
        self.base_url = "https://api.deepseek.com/beta"
        self.model_name = "deepseek-coder"

        # self.api_key = "sk-1234"
        # self.base_url = "http://api.openai.rnd.huawei.com/v1"
        # self.model_name = "qwen3-32b"

        self.cache_dir = "./cache"
        self.final_project_dir = "./final_project"