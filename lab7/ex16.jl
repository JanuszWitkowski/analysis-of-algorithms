#!/usr/local/bin/julia
counter = 0

function f(n::UInt64)
    # Counting
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

function generating_function(n::UInt64)::UInt128
    if n < 2
        return 1
    else
        return 2 + sum(map(x -> generating_function(x), [i for i in 1:n-1]))
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

println("n\t|theory\t|gen\t|ctr\t|error\t(m=$m)")
println("----------------------------------------")
for n in ns
    c_acc = 0
    for _ in experiments_range
        f(n)
        c_acc += counter
        global counter
        counter = 0
    end
    c_value = c_acc / m
    c_rounded = round(c_value, digits=1)
    t_value = theory(n)
    g_value = generating_function(n)
    err = abs(c_value - t_value)/t_value
    err_rounded = round(err, digits=5)
    # println("$n\t|$t_value\t|$g_value\t|$c_value\t|$err")
    println("$n\t|$t_value\t|$g_value\t|$c_rounded\t|$err_rounded")
end

