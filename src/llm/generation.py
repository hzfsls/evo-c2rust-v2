from tqdm import tqdm
from entity.metadata import RustCode
from llm.client import GenerationClient
from cache.cache import ProjectCache

def merge_prompt(prompt, c_code):
    return prompt + f"""\
Source:
```c
{c_code.strip()}
```

Translation:
```rust
"""

def merge_repair_prompt(prompt, c_code, compiler_msg):
    return prompt + f"""\
Source:
```rust
{c_code.strip()}
```

Error Message:{compiler_msg}

Fixed Code:
```rust
"""

def get_delim_repair_candidates(client, code, compiler_msg):
    text = merge_repair_prompt(client.config.delim_repair_prompt, code, compiler_msg)
    response = client.get_response(text)
    return [response]


def get_repair_candidates(client, code, compiler_msg):
    text = merge_repair_prompt(client.config.repair_prompt, code, compiler_msg)
    response = client.get_response(text)
    return [response]

def get_llm_gen_result(client, code, prompt):
    text = merge_prompt(prompt, code)
    response = client.get_response(text)
    return response

def update_codes(client, typ, codes: list[RustCode], cache: ProjectCache):
    prompts = {
        "definition": client.config.definition_prompt,
        "macro": client.config.macro_prompt,
        "macro_function": client.config.macro_function_prompt,
        "dummy_function": client.config.dummy_function_prompt,
        "function": client.config.function_prompt,
    }
    prompt = prompts[typ]
    for c in tqdm(codes):
        if c.c_code in cache:
            c.rust_code = cache.get(typ, c.c_code)
            continue
        c.rust_code = get_llm_gen_result(client, c.c_code, prompt)
        if cache is not None:
            cache.update(typ, c.c_code, c.rust_code)
