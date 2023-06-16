#!/usr/local/bin/julia
counter = 0

function f(n::Int128)::Int128
    s = 0
    if n == 0
        return 1
    else
        for i in 0:n-1
            s += f(i)   # line 6
            # Counting
            global counter
            counter += 1
        end
        return s
    end
end

function theory(n::Int128)::Int128
    return (2^n) - 1
end


n_range = 16
if length(ARGS) > 0
    n_range = parse(Int128, ARGS[1])
end
ns = 0:n_range
for n in ns
    s_value = f(n)
    c_value = counter
    t_value = theory(n)
    global counter
    counter = 0
    println("n = $n: ctr = $c_value; s = $s_value")
    if c_value != t_value
        println("ERROR: theory=$t_value != $c_value=ctr !!!")
        break;
    end
end

