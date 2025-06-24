# 需要安装 tree-grepper bc

function count_safe_ratio() {
    a=$(tree-grepper -q rust '((function_item (function_modifiers) @_m)@f (#match? @_m "unsafe"))' | grep ':f:' | wc -l)
    b=$(tree-grepper -q rust '((function_item)@f)' | grep ':f:' | wc -l)
    echo $a
    echo $b
    ratio=$(echo "scale=4; 100 - $a * 100 / $b" | bc)  # 先计算浮点数
    rounded_ratio=$(printf "%.2f" "$ratio")            # 四舍五入保留两位
    echo "// Safe API ratio is: ${rounded_ratio}%"
    a=$(tree-grepper -q rust '((unsafe_block)@b)' | sed -e 's/^.*:b://' | wc -c)
    c=$(tree-grepper -q rust '((function_item (function_modifiers) @_m)@f (#match? @_m "unsafe"))' | sed -e 's/^.*:f://' | wc -c)
    b=$(tree-grepper -q rust '((function_item))' | sed -e 's/^.*:f://' | wc -c)
    echo $a
    echo $c
    echo $b
    # printf // Safe Code ratio is: $(( 100 - (a + c) * 100 / b )) %
    ratio=$(echo "scale=4; 100 - ($a + $c) * 100 / $b" | bc)  # 先计算浮点数
    rounded_ratio=$(printf "%.2f" "$ratio")            # 四舍五入保留两位
    echo "// Safe API ratio is: ${rounded_ratio}%"
}
count_safe_ratio