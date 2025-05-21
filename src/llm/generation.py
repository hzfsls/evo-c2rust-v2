from openai import OpenAI
from pebble import ProcessPool
from httpx import Client
from tqdm import tqdm

import multiprocessing

import sys

from misc.exceptions import CallLLMTimeoutError
from logger.logger import setup_logger
from rust_metadata.classes import RustCode

#第二种
import json
import requests
import time
import re
MAX_RETRIES = 3 # 尝试次数

# 全局禁用 veriy=False warning
# todo：实际生产环境下需要优化
from urllib3.exceptions import InsecureRequestWarning
requests.packages.urllib3.disable_warnings(InsecureRequestWarning)

delim_repair_prompt = """\
Fix the compiler bugs in the following Rust code with provided compiler error messagesm, possibly because of mismatched parens.
Only fix lines that have unmatched parens bugs, don't modify any other code.

Source:
```rust
pub fn rb_tree_insert_case4(mut tree: Ptr<RBTree>, mut node: Ptr<RBTreeNode>) {
    let mut next_node: Ptr<RBTreeNode> = Default::default();
    let mut side: RBTreeNodeSide = Default::default();
    
    side = rb_tree_node_side(node);
    if (side != rb_tree_node_side(node.parent) {
        next_node = node.parent;
        rb_tree_rotate(tree, node.parent, (1 - side));
    } else {
        next_node = node;
    }
    rb_tree_insert_case5(tree, next_node);
}
```

Error Message:
"error: mismatched closing delimiter: `}`   --> src/src/rb_tree_c.rs:154:8\n    |\n148 | pub fn rb_tree_insert_case4(mut tree: Ptr<RBTree>, mut node: Ptr<RBTreeNode>) {\n    |                                                                               - closing delimiter possibly meant for this\n...\n154 |     if (side != rb_tree_node_side(node.parent).cast().as_bool() {\n    |        ^ unclosed delimiter\n...\n163 | }\n    | ^ mismatched closing delimiter\n\nerror: could not compile `my_proj` (lib) due to 1 previous error\n"

Fixed Code:
```rust
pub fn rb_tree_insert_case4(mut tree: Ptr<RBTree>, mut node: Ptr<RBTreeNode>) {
    let mut next_node: Ptr<RBTreeNode> = Default::default();
    let mut side: RBTreeNodeSide = Default::default();
    
    side = rb_tree_node_side(node);
    if (side != rb_tree_node_side(node.parent)) {
        next_node = node.parent;
        rb_tree_rotate(tree, node.parent, (1 - side));
    } else {
        next_node = node;
    }
    rb_tree_insert_case5(tree, next_node);
}
```

Source:
```rust
pub fn rb_tree_insert(mut tree: Ptr<RBTree>, mut key: RBTreeKey, mut value: RBTreeValue) -> Ptr<RBTreeNode> {
    let mut node: Ptr<RBTreeNode>;
    let mut rover: Ptr<Ptr<RBTreeNode>>;
    let mut parent: Ptr<RBTreeNode>;
    let mut side: RBTreeNodeSide;
    
    node = c_malloc!(c_sizeof!(RBTreeNode));
    
    if (node == NULL!()) {
        return NULL!();
    }
    node.key = key;
    node.value = value;
    node.color = RB_TREE_NODE_RED!();
    node.children[RB_TREE_NODE_LEFT!()] = NULL!();
    node.children[RB_TREE_NODE_RIGHT!()] = NULL!();
    
    parent = NULL!();
    rover = c_ref!(tree.root_node);
    
    while (*rover != NULL!()) {
        parent = *rover;
        if (tree.compare_func(value, (*rover).value) < 0 {
            side = RB_TREE_NODE_LEFT!();
        } else {
            side = RB_TREE_NODE_RIGHT!();
        }
        
        rover = c_ref!((*rover).children[side]);
    }
    
    *rover = node;
    node.parent = parent;
    rb_tree_insert_case1(tree, node);
    tree.num_nodes.prefix_plus_plus();
    return node;
}
```

Error Message:
"error: mismatched closing delimiter: `}`\n   --> src/src/rb_tree_c.rs:194:12\n    |\n191 |     while (*rover != NULL!()).as_bool() {\n    |                                         - closing delimiter possibly meant for this\n...\n194 |         if (tree.compare_func(value.cast(), (*rover).value.cast()) < 0 {\n    |            ^ unclosed delimiter\n...\n201 |     }\n    |     ^ mismatched closing delimiter\n\nerror: could not compile `my_proj` (lib) due to 1 previous error\n"

Fixed Code:
```rust
pub fn rb_tree_insert(mut tree: Ptr<RBTree>, mut key: RBTreeKey, mut value: RBTreeValue) -> Ptr<RBTreeNode> {
    let mut node: Ptr<RBTreeNode>;
    let mut rover: Ptr<Ptr<RBTreeNode>>;
    let mut parent: Ptr<RBTreeNode>;
    let mut side: RBTreeNodeSide;
    
    node = c_malloc!(c_sizeof!(RBTreeNode));
    
    if (node == NULL!()) {
        return NULL!();
    }
    node.key = key;
    node.value = value;
    node.color = RB_TREE_NODE_RED!();
    node.children[RB_TREE_NODE_LEFT!()] = NULL!();
    node.children[RB_TREE_NODE_RIGHT!()] = NULL!();
    
    parent = NULL!();
    rover = c_ref!(tree.root_node);
    
    while (*rover != NULL!()) {
        parent = *rover;
        if (tree.compare_func(value, (*rover).value) < 0) {
            side = RB_TREE_NODE_LEFT!();
        } else {
            side = RB_TREE_NODE_RIGHT!();
        }
        
        rover = c_ref!((*rover).children[side]);
    }
    
    *rover = node;
    node.parent = parent;
    rb_tree_insert_case1(tree, node);
    tree.num_nodes.prefix_plus_plus();
    return node;
}
```
"""


repair_prompt = """\
Fix the compiler bugs in the following Rust code with provided compiler error messagesm.
Fix these bugs according to the compiler informations:
1. Type mismatch: use `cast::<T>` to cast to original type to the targeted type.
2. Wrong function as struct field calling: `my_struct.my_func(a, b)` should be corrected as `(my_struct.my_func)(a, b)`.
3. Constant/Macro confusion: `a > MY_MACRO` should be `a > MY_MACRO!()`, and `b > my_constant!()` should be `b > my_constant`
4. Other bugs, just repair the corresponding line with the reference of error messages. 

Source:
```rust
pub fn binomial_tree_unref(mut tree: Ptr<BinomialTree>) {
    let mut i: i32 = Default::default();

    if (tree == NULL!()) {
        return;
    }
    
    tree.refcount.suffix_minus_minus()
    
    if (tree.refcount == 0) {
        c_for!(i = 0; i < tree.order; i.prefix_plus_plus(); {
            binomial_tree_unref(tree.subtrees[i]);
        });
        c_free!(tree.subtrees);
        c_free!(tree);
    }
}
```

Error Message:
"error[E0308]: mismatched types\n  --> src/src/binomial_heap_c.rs:55:27\n   |\n55 |         c_for!(i = 0; i < tree.order; i.prefix_plus_plus(); {\n   |                       -   ^^^^^^^^^^ expected `i32`, found `u16`\n   |                       |\n   |                       expected because this is `i32`\n   |\nhelp: you can convert a `u16` to an `i32`\n   |\n55 |         c_for!(i = 0; i < tree.order.into(); i.prefix_plus_plus(); {\n   |                                     +++++++\n\nFor more information about this error, try `rustc --explain E0308`.\nerror: could not compile `my_proj` (lib) due to 1 previous error\n"

Fixed Code:
```rust
pub fn binomial_tree_unref(mut tree: Ptr<BinomialTree>) {
    let mut i: i32 = Default::default();

    if (tree == NULL!()) {
        return;
    }
    
    tree.refcount.suffix_minus_minus()
    
    if (tree.refcount == 0) {
        c_for!(i = 0; i < tree.order.cast::<i32>(); i.prefix_plus_plus(); {
            binomial_tree_unref(tree.subtrees[i]);
        });
        c_free!(tree.subtrees);
        c_free!(tree);
    }
}
```

Source:
```rust
pub fn rb_tree_insert(mut tree: Ptr<RBTree>, mut key: RBTreeKey, mut value: RBTreeValue) -> Ptr<RBTreeNode> {
    let mut node: Ptr<RBTreeNode>;
    let mut rover: Ptr<Ptr<RBTreeNode>>;
    let mut parent: Ptr<RBTreeNode>;
    let mut side: RBTreeNodeSide;

    node = c_malloc!(c_sizeof!(RBTreeNode));

    if (node == NULL!()) {
        return NULL!();
    }

    node.key = key;
    node.value = value;
    node.color = RB_TREE_NODE_RED!();
    node.children[RB_TREE_NODE_LEFT!()] = NULL!();
    node.children[RB_TREE_NODE_RIGHT!()] = NULL!();

    parent = NULL!();
    rover = c_ref!(tree.root_node);

    while (*rover != NULL!()) {
        parent = *rover;

        if (tree.compare_func(value, (*rover).value) < 0) {
            side = RB_TREE_NODE_LEFT!();
        } else {
            side = RB_TREE_NODE_RIGHT!();
        }

        rover = c_ref!((*rover).children[side]);
    }

    *rover = node;
    node.parent = parent;

    rb_tree_insert_case1(tree, node);

    tree.num_nodes.prefix_plus_plus();

    return node;
}
```

Error Message:
"error: mismatched closing delimiter: `}`\n   --> src/src/rb_tree_c.rs:194:12\n    |\n191 |     while (*rover != NULL!()).as_bool() {\n    |                                         - closing delimiter possibly meant for this\n...\n194 |         if (tree.compare_func(value.cast(), (*rover).value.cast()) < 0 {\n    |            ^ unclosed delimiter\n...\n201 |     }\n    |     ^ mismatched closing delimiter\n\nerror: could not compile `my_proj` (lib) due to 1 previous error\n"

Fixed Code:
```rust
pub fn rb_tree_insert(mut tree: Ptr<RBTree>, mut key: RBTreeKey, mut value: RBTreeValue) -> Ptr<RBTreeNode> {
    let mut node: Ptr<RBTreeNode>;
    let mut rover: Ptr<Ptr<RBTreeNode>>;
    let mut parent: Ptr<RBTreeNode>;
    let mut side: RBTreeNodeSide;

    node = c_malloc!(c_sizeof!(RBTreeNode));

    if (node == NULL!()) {
        return NULL!();
    }

    node.key = key;
    node.value = value;
    node.color = RB_TREE_NODE_RED!();
    node.children[RB_TREE_NODE_LEFT!()] = NULL!();
    node.children[RB_TREE_NODE_RIGHT!()] = NULL!();

    parent = NULL!();
    rover = c_ref!(tree.root_node);

    while (*rover != NULL!()) {
        parent = *rover;

        if ((tree.compare_func)(value, (*rover).value) < 0) {
            side = RB_TREE_NODE_LEFT!();
        } else {
            side = RB_TREE_NODE_RIGHT!();
        }

        rover = c_ref!((*rover).children[side]);
    }

    *rover = node;
    node.parent = parent;

    rb_tree_insert_case1(tree, node);

    tree.num_nodes.prefix_plus_plus();

    return node;
}
```

Source:
```rust
pub fn string_hash(mut string: Ptr<Void>) -> u32 {
    let mut result: u32 = 5381;
    let mut p: Ptr<u8> = string.cast::<Ptr<u8>>();

    while (*p != '\\0') {
        result = (result << 5) + result + (*p).cast::<u32>();
        p += 1;
    }

    return result;
}
```

Error Message:
"error[E0308]: mismatched types\n --> src/src/hash_string_c.rs:8:18\n  |\n8 |     while (*p != '\\0') {\n  |            --    ^^^^ expected `u8`, found `char`\n  |            |\n  |            expected because this is `u8`\n  |\nhelp: if you meant to write a byte literal, prefix with `b`\n  |\n8 |     while (*p != b'\\0') {\n  |                  ~~~~~\n\nFor more information about this error, try `rustc --explain E0308`.\nerror: could not compile `my_proj` (lib) due to 1 previous error\n"

Fixed Code:
```rust
pub fn string_hash(mut string: Ptr<Void>) -> u32 {
    let mut result: u32 = 5381;
    let mut p: Ptr<u8> = string.cast::<Ptr<u8>>();

    while (*p != b'\\0' as u8) {
        result = (result << 5) + result + (*p).cast::<u32>();
        p += 1;
    }

    return result;
}
```

Source:
```rust
pub fn trie_free_list_pop(mut list: Ptr<Ptr<TrieNode>>) -> Ptr<TrieNode> {
    let mut result: Ptr<TrieNode>;

    result = *list;
    *list = result.data;

    return result;
}
```

Error Message:
"error[E0308]: mismatched types\n  --> src/src/trie_c.rs:46:13\n   |\n46 |     *list = result.data;\n   |     -----   ^^^^^^^^^^^ expected `Ptr<_TrieNode>`, found `Ptr<u8>`\n   |     |\n   |     expected due to the type of this binding\n   |\n   = note: expected struct `memory::ptr::Ptr<_TrieNode>`\n              found struct `memory::ptr::Ptr<u8>`\nhelp: consider removing the tuple struct field `data`\n   |\n46 -     *list = result.data;\n46 +     *list = result;\n   |\n\nFor more information about this error, try `rustc --explain E0308`.\nerror: could not compile `my_proj` (lib) due to 1 previous error\n"

Fixed Code:
```rust
pub fn trie_free_list_pop(mut list: Ptr<Ptr<TrieNode>>) -> Ptr<TrieNode> {
    let mut result: Ptr<TrieNode>;

    result = *list;
    *list = result.data.cast::<Ptr<TrieNode>>();

    return result;
}
```

Source:
```rust
pub fn set_allocate_table(mut set: Ptr<Set>) -> i32 {
    if (set.prime_index < set_num_primes!()) {
        let tmp0 = set.prime_index;
        set.table_size = set_primes[tmp0];
    } else {
        set.table_size = (set.entries * 10);
    }
    set.table = c_calloc!(set.table_size, c_sizeof!(Ptr<SetEntry>));
    return (set.table != NULL!()).cast::<i32>();
}
```

Error Message:
"error: cannot find macro `set_num_primes` in this scope\n  --> src/src/set_c.rs:35:27\n   |\n35 |     if (set.prime_index < set_num_primes!()) {\n   |                           ^^^^^^^^^^^^^^\n   |\n   = note: `set_num_primes` is in scope, but it is a constant, not a macro\n\nerror: could not compile `my_proj` (lib) due to 1 previous error\n"

Fixed Code:
```rust
pub fn set_allocate_table(mut set: Ptr<Set>) -> i32 {
    if (set.prime_index < set_num_primes) {
        let tmp0 = set.prime_index;
        set.table_size = set_primes[tmp0];
    } else {
        set.table_size = (set.entries * 10);
    }
    set.table = c_calloc!(set.table_size, c_sizeof!(Ptr<SetEntry>));
    return (set.table != NULL!()).cast::<i32>();
}
```
"""

 
logger = setup_logger("generation.py")

class GenerationClient:
    # def __init__(self, api_key, base_url="https://api.deepseek.com/beta", type=0, proxy=None):
    def __init__(self, api_key, base_url="", proxy=None, typenum=1,
                    url = "", app_id = "", 
                    app_static_token = "", enterprise = "",
                    account = "", scenario_uuid = ""):
        #模型分类 0为第一种，1为第二种
        self.type = typenum
        #第一种用法
        self.api_key = api_key
        self.base_url = base_url
        self.proxy = proxy
        #第二种用法,内部模型参数
        self.url = url
        self.app_id = app_id
        self.app_static_token = app_static_token
        self.enterprise = enterprise
        self.account = account
        self.scenario_uuid = scenario_uuid

    #第二种用法前置请求
    def get_his3_token(self,env):
        """
        :param url:
        :param app_id:
        :param app_static_token:
        :return:
        """
        if env == "pro":
            url = self.url
            app_id = self.app_id
            app_static_token = self.app_static_token
            enterprise = self.enterprise
            account = self.account 
        else:
            url = self.url
            app_id = self.app_id
            app_static_token = self.app_static_token
            enterprise = self.enterprise
            account = "com.huawei.doprafsp"
        data = {
            "data": {
                "type": "JWT-Token",
                "attributes": {
                    "project": app_id,
                    "secret": app_static_token,
                    "method": "CREATE",
                    "account": account,
                    "enterprise": enterprise
                }
            }
        }
        response = requests.post(url, json=data, verify=False)
        result = json.loads(response.text)
        if not result["access_token"]:
            raise ValueError("用户认证信息错误，请检查app_id和app_token!!!")
        dynamic_token = result["access_token"]
        return dynamic_token

    def get_response(self, text):
        if self.type == 0:
            openai_client = OpenAI(
                api_key=self.api_key,
                base_url=self.base_url,
                http_client=Client(
                    # proxy=None,
                    verify=False
                )   
            )
            response = openai_client.chat.completions.create(
                model="deepseek-coder",
                messages=[
                    {"role": "assistant", "content": text, "prefix": True},
                ],
                stop=["```"],
                temperature=0,
                top_p=0.01,
                max_tokens=4096,
                stream=False,
            )
            result = response.choices[0].message.content
            return result
        else :#第二种用法
            question = text
            for attempt in range(MAX_RETRIES):
                try:
                    scenario_uuid = self.scenario_uuid
                    env = "pro"
                    if env == "pro":
                        url = "https://chatbot.his.huawei.com/aigc-api-gateway/aigc-model-gateway/dialogue"
                    elif env == "beta" or env == "uat":
                        url = "https://chatbot.his-beta.huawei.com/aigc-api-gateway/aigc-model-gateway/dialogue"
                    else:
                        url = "https://console-sit.his-op-beta.huawei.com/aigc-api-gateway/aigc-model-gateway/dialogue"

                    token = self.get_his3_token(env)
                    headers = {
                        "Content-Type": "application/json",
                        "Authorization": token
                    }
                    post_params = {
                        "question": question,
                        "scenarioUuid": scenario_uuid,  # 拼接场景uuid
                    }
                    response = requests.post(url=url, headers=headers, json=post_params, verify=False)

                    # 解析 JSON 响应
                    response_dict = response.json()  # 直接解析 JSON
                    error_code = response_dict.get("errorCode", -1)  # 如果没有 errorcode，默认-1

                    if error_code == 0:
                        result = response_dict["choices"][0]["content"]
                        # Todo：清理因为大模型产生的冗余代码，取出rust代码，考虑在后续提出更好的处理大模型幻觉方法
                        pattern = re.compile(r'(```rust|```)(.*?)(```|$)', re.DOTALL)
                        result = pattern.sub(lambda m: m.group(2).strip(), result)
                        return result
                    else:
                        logger.error(f"API 返回错误: errorcode={error_code}, message={response_dict.get('message', '未知错误')}")
                        sys.exit(1)

                except requests.exceptions.RequestException as e:
                    logger.warning(f"网络错误（可能是算力紧张）: {e}")
                    wait_time = 5 ** attempt
                except Exception as e:
                    logger.warning("发生未知错误:{e}")
                    wait_time = 2 ** attempt

                # 失败后等待一段时间再重试
                logger.info(f"重试 {attempt + 1}/{MAX_RETRIES}，等待 {wait_time} 秒...")
                time.sleep(wait_time)
            
            logger.error(f"多次重试失败，放弃请求")
            sys.exit(1)

def merge_prompt(prompt, c_code):
    return "You can't have any natural language descriptions in your code reply.No extra output Rust definition."+prompt + f"""\
Source:
```c
{c_code.strip()}
```

Translation:
```rust
"""

def merge_repair_prompt(prompt, c_code, compiler_msg):
    return "You can't have any natural language descriptions in your code reply.No extra output Rust definition."+prompt + f"""\
Source:
```rust
{c_code.strip()}
```

Error Message:{compiler_msg}

Fixed Code:
```rust
"""

def get_delim_repair_candidates(client, code, compiler_msg):
    text = merge_repair_prompt(delim_repair_prompt, code, compiler_msg)
    response = client.get_response(text)
    return [response]


def get_repair_candidates(client, code, compiler_msg):
    text = merge_repair_prompt(repair_prompt, code, compiler_msg)
    response = client.get_response(text)
    return [response]

def get_llm_generated_result(client, code, prompt, cache = {}):
    if code in cache:
        return cache[code]
    text = merge_prompt(prompt, code)
    response = client.get_response(text)
    return response

def update_codes(type, client, prompts, codes: list[RustCode], caches = {}):
    cache = caches[type]
    prompt = prompts[type]
    # for c in tqdm(codes):
    #     c.rust_code = get_llm_generated_result(client, c.c_code, prompt, cache.cache)
    #     print(c.rust_code)
    #     cache.update(c.c_code, c.rust_code)
    if len(codes) == 1:
        c = codes[0]
        c.rust_code = get_llm_generated_result(client, c.c_code, prompt, cache.cache)
        cache.update(c.c_code, c.rust_code)
    else:
        # 区分操作系统,window使用原来多进程方法会产生问题，为window添加了其他多进程方法
        if sys.platform.startswith('win'):
            with multiprocessing.Pool(5) as pool:
                futures = []
                for c in codes:
                    future = pool.apply_async(
                        get_llm_generated_result, args=[client, c.c_code, prompt, cache.cache]
                    )
                    futures.append((c, future))
                for c, future in tqdm(futures):
                    try:
                        rust_code = future.get()
                        c.rust_code = rust_code
                        cache.update(c.c_code, c.rust_code)
                    except Exception as e:
                        raise CallLLMTimeoutError(e)
                pool.close()
                pool.join()
        elif sys.platform.startswith('linux'):
            with ProcessPool(max_workers=5) as pool:
                futures = []
                for c in codes:
                    future = pool.schedule(
                        get_llm_generated_result, args=[client, c.c_code, prompt, cache.cache], timeout=300
                    )
                    futures.append((c, future))
                for c, future in tqdm(futures):
                    try:
                        rust_code = future.result()
                        c.rust_code = rust_code
                        cache.update(c.c_code, c.rust_code)
                    except Exception as e:
                        raise CallLLMTimeoutError(e)