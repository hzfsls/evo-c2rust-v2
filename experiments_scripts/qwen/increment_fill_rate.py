from config.global_config import GlobalConfig
from metadata_extraction.c_metadata import extract_c_metadata_from_project
from metadata_extraction.rust_metadata import c_metadata_to_rust_metadata
from cache.cache import ProjectCache
from code_optim.code_gen import  code_generation, code_verification
from code_optim.code_optim import code_optimization
from code_optim.predefined_agents import (
    get_fix_mismatched_delim_agent,
    get_implicit_casting_removal_agent,
    get_as_bool_removal_agent,
    get_struct_index_advancement_agent,
    get_definition_replace_agent,
    get_llm_repair_agent,
)

from llm.client_hw import GenerationClient


import argparse

definition_prompt = """\
Translate the C Code to Rust. 
You need to translate the definition only.
Notice that: 
You need to translate `void*` type to VoidPtr in Rust, and all char type to u8.
Array in C like int[10] should be translated to `Array` type in Rust: Array<i32, 10>, and you should use arr! macro to initialize the array, for example, `int a[5] = {1, 2, 3, 4, 5};` should be translated to `a: Array<i32, 5> = arr![1, 2, 3, 4, 5];`.
Enum Type in C should be translated to i32 and the enum values should be translated to macro_rules, and all translated macros in Rust should be uppercased.
Pointers in C should be translated to Ptr<T> in Rust.
Remember when translating macros, add `pub(crate)` to the macro definition to make it visible.
You should translate the global variables start with g_ with Global<T> type and global!() macro, for example, `static int g_a = 0;` should be translated to `pub static g_a: Global<i32> = global!(0);`. However, if it not not start with g_, just translate it to a constant.

Here are some examples:
Source:
```c
typedef void *MY_VALUE;
```
Translation:
```rust
pub type MyValue = VoidPtr;
```
Source:
```
#define MY_NULL 0
```
Translation:
```rust
macro_rules! MY_NULL { () => { NULL!() } }
pub(crate) use MY_NULL;
```

Source:
```c
typedef struct MyStruct MY_STRUCT1;
```

Translation:
```rust
pub type MY_STRUCT1 = MyStruct;
```

Source:
```c
typedef enum
{
    MY_RED = 0,
    MY_GREEN,
    MY_BLUE
} MyEnum;
```

Translation:
```rust
pub type MyEnum = i32;
macro_rules! MY_RED { () => { 0 } }
pub(crate) use MY_RED;
macro_rules! MY_GREEN { () => { 1 } }
pub(crate) use MY_GREEN;
macro_rules! MY_BLUE { () => { 2 } }
pub(crate) use MY_BLUE;
```

Source:
```c
typedef int (*MyFunction)(int a, int b);
```

Translation:
```rust
pub type MyFunction = FuncPtr<fn(i32, i32) -> i32>;
```

Source:
```c
typedef void (*ANO_function)(const void* a, char* b);
```

Translation:
```rust
pub type ANO_function = FuncPtr<fn(VoidPtr, Ptr<u8>)>;
```

Source:
```c
static MyFunction g_MyCustomFunc = NULL;
```

Translation:
```rust
pub static g_MyCustomFunc: Global<MyFunction> = global!(NULL!());
```

When translating string literals in C, use cstr! macro

Source:
```c
const char* g_MyGlobalStr = "Hello, World!";
```

Translation:
```rust
pub static g_MyGlobalStr: Global<Ptr<u8>> = global!(cstr!("Hello, World!"));
```

Source:
```c
static MyFunction g_MyCustomFunc = NULL;
```

Source:
```c
int[] g_MyCustomArray = {1, 2, 3, 4, 5};
const int[] myCustomArray = {1, 2, 3, 4, 5};
```

Translation:
```rust
pub static g_MyCustomArray: Global<Array<i32, 5>> = global!(arr![1, 2, 3, 4, 5]);
pub const myCustomArray: Array<i32, 5>> = arr![1, 2, 3, 4, 5];
```

Source:
```c
static int[] g_A10 = {3, 4, 5, 6, 7, 8, 9, 10, 11, 12};
static const int[] A10 = {3, 4, 5, 6, 7, 8, 9, 10, 11, 12};
```

Translation:
```
pub static g_A10: Global<Array<i32, 10>> = global!(arr![3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
pub const A10: Array<i32, 10> = arr![3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
```

Source:
```c
static const int arr_counts = sizeof(arr) / sizeof(int);
```

Translation:
```
pub const arr_counts: i32 = arr.len() as i32;
```

Source:
```c
typedef struct {
    int arr[2];
    unsigned int length;
    MySimpleStruct* ss; 
} MySimpleStruct;
```

Translation:
```rust
#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct MySimpleStruct {
    pub arr: Array<i32, 2>,
    pub length: u32,
    pub ss: Ptr<MySimpleStruct>,
}
```

Source:
```c
struct MySimpleStruct {
    int* arr;
    unsigned int length;
};
```

Translation:
```rust
#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct MySimpleStruct {
    pub arr: Ptr<i32>,
    pub length: u32,
}
```

Source:
```c
typedef struct _MyComplexStruct {
    MyStructEntry **vEntries;
    const char* vlength;
    MyStructValueFunc valueFunc;
    int values[64];
    FILE* file;
	MyStructNode *children[CHINDREN_SIZE];
    MyStructNode more_children[CHINDREN_SIZE * 5 + 1];
} MY_Com_Struct;
```

Translation:
```rust
#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct MY_Com_Struct {
    pub vEntries: Ptr<Ptr<MyStructEntry>>,
    pub vlength: Ptr<u8>,
    pub valueFunc: MyStructValueFunc,
    pub values: Array<i32, 64>,
    pub file: FilePtr,
    pub children: Array<Ptr<MyStructNode>, { CHINDREN_SIZE!() }>,
    pub more_children: Array<MyStructNode, { CHINDREN_SIZE!() * 5 + 1 }>,
}
```
"""

if __name__ == "__main__":
    projects = ["avl", "bzp", "cmptlz", "rapidlz", "md5", "sha256"]
    method_report = {}
    all_proj_total_cnt = 0
    all_proj_passed_cnt = 0
    all_proj_function_cnt = 0
    all_proj_function_passed_cnt = 0

    for project_name in projects:          
        config = GlobalConfig()
        config.project_dir = "./data/increment/project"
        config.c_metadata_dir = "./data/increment/c_metadata"
        config.rust_metadata_dir = "./data/increment/rust_metadata"
        config.project_name = project_name

        config.definition_prompt = definition_prompt

        extract_c_metadata_from_project(config)
        rust_metadata = c_metadata_to_rust_metadata(config)

        config.api_key = "sk-1234"
        config.base_url = "http://api.openai.rnd.huawei.com/v1"
        config.model_name = "qwen3-32b"
        client = GenerationClient(config)

        cache_0 = ProjectCache(config, cache_dir="./data/increment/cache/evo-c2rust-v2-qwen-gen-only")
        cache_1 = ProjectCache(config, cache_dir="./data/increment/cache/evo-c2rust-v2-qwen-delim-fix")
        cache_2 = ProjectCache(config, cache_dir="./data/increment/cache/evo-c2rust-v2-qwen-rule-fix")

        code_generation(config, rust_metadata, cache_0, client, multi_process=False)
        rust_metadata = c_metadata_to_rust_metadata(config)
        report_0 = code_verification(config, rust_metadata, cache_0)

        rust_metadata = c_metadata_to_rust_metadata(config)
        FIX_MISMATCHED_DELIM_AGENT = get_fix_mismatched_delim_agent(config, rust_metadata, client)
        report_1 = code_optimization(config, rust_metadata, cache_0, cache_1, client,
            optimizations={
                "macro": [FIX_MISMATCHED_DELIM_AGENT],
                "macro_function": [FIX_MISMATCHED_DELIM_AGENT],
                "definition": [FIX_MISMATCHED_DELIM_AGENT],
                "function": [FIX_MISMATCHED_DELIM_AGENT],
            })

        rust_metadata = c_metadata_to_rust_metadata(config)
        IMPLICIT_CASTING_REMOVAL_AGENT = get_implicit_casting_removal_agent(config, rust_metadata)
        AS_BOOL_REMOVAL_AGENT = get_as_bool_removal_agent(config, rust_metadata)
        STRUCT_INDEX_ADVANCEMENT_AGENT = get_struct_index_advancement_agent(config, rust_metadata)
        DEFINITION_REPLACE_AGENT = get_definition_replace_agent(config, rust_metadata)
        report_2 = code_optimization(config, rust_metadata, cache_1, cache_2, client,
            optimizations={
                "definition": [DEFINITION_REPLACE_AGENT],
                "function": [IMPLICIT_CASTING_REMOVAL_AGENT, AS_BOOL_REMOVAL_AGENT, STRUCT_INDEX_ADVANCEMENT_AGENT],
            })
    

        items = ["macro", "macro_function", "definition", "function"]
        calcs = ["all_cnt", "passed_cnt", "pass_rate"]
        proj_report = {item: {calc: report_2[item][calc] for calc in calcs} for item in items}
        total_all_cnt = sum(proj_report[item]["all_cnt"] for item in items)
        total_passed_cnt = sum(proj_report[item]["passed_cnt"] for item in items)
        total_passed_rate = total_passed_cnt / total_all_cnt
        all_proj_total_cnt += total_all_cnt
        all_proj_passed_cnt += total_passed_cnt
        all_proj_function_cnt += proj_report["function"]["all_cnt"]
        all_proj_function_passed_cnt += proj_report["function"]["passed_cnt"]
        proj_report["total"] = {"all_cnt": total_all_cnt, "passed_cnt": total_passed_cnt, "pass_rate": total_passed_rate}
        method_report[project_name] = proj_report
    all_proj_total_pass_rate = all_proj_passed_cnt / all_proj_total_cnt
    all_proj_function_pass_rate = all_proj_function_passed_cnt / all_proj_function_cnt
    method_report["all"] = {"total" : {"all_cnt": all_proj_total_cnt, "passed_cnt": all_proj_passed_cnt, "pass_rate": all_proj_total_pass_rate}, "function": {"all_cnt": all_proj_function_cnt, "passed_cnt": all_proj_function_passed_cnt, "pass_rate": all_proj_function_pass_rate},
    }
    with open("report.json", "w") as f:
        import json
        json.dump(method_report, f, indent=4)