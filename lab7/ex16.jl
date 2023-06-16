#!/usr/local/bin/julia
counter = 0

function f(n::UInt64)
    global counter
    counter += 1
    if n >= 2
        for k in 1:n
            if rand(Bool)
                f(k)
            end
        end
    end
end

function theory(n::UInt64)::UInt128
    if n < 2
        return 1
    else
        return 3 * ( 2^(n-2) )
    end
end


n_max = 16
m = UInt128(100)
if length(ARGS) > 0
    n_max = parse(UInt64, ARGS[1])
    if length(ARGS) > 1
        m = parse(UInt128, ARGS[2])
    end
end
ns = 0:n_max
experiments_range = 1:m
for n in ns
    c_acc = 0
    for _ in experiments_range
        f(n)
        c_acc += counter
        global counter
        counter = 0
    end
    c_value = c_acc / m
    t_value = theory(n)
    println("n = $n: ctr = $c_value; theory = $t_value")
end
