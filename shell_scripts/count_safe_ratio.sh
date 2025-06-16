tinfunction count_safe_ratio() {
    a=$(tree-grepper -q rust '((function_item (function_modifiers) @_m)@f (#match? @_m "unsafe"))' | grep ':f:' | wc -l)
    b=$(tree-grepper -q rust '((function_item)@f)' | grep ':f:' | wc -l)
    echo // Safe API ratio is: $(( 100 - a * 100 / b )) %
    a=$(tree-grepper -q rust '((unsafe_block)@b)' | sed -e 's/^.*:b://' | wc -c)
    c=$(tree-grepper -q rust '((function_item (function_modifiers) @_m)@f (#match? @_m "unsafe"))' | sed -e 's/^.*:f://' | wc -c)
    b=$(tree-grepper -q rust '((function_item))' | sed -e 's/^.*:f://' | wc -c)
    echo // Safe Code ratio is: $(( 100 - (a + c) * 100 / b )) %
}
count_safe_ratio