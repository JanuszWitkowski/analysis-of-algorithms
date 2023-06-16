#!/usr/local/bin/julia
counter = 0

function f(n::Int64)
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


n_range = 16
if length(ARGS) > 0
    n_range = parse(Int64, ARGS[1])
end
ns = 0:n_range
for n in ns
    s_value = f(n)
    c_value = counter
    global counter
    counter = 0
    println("n = $n: ctr = $c_value; s = $s_value")
end

