#!/usr/local/bin/julia
include("stationary.jl")

function calculate_approximate(p_matrix, epsilon)
    (n, _) = size(p_matrix)
    stat_dist = stationary_distribution(p_matrix)
    P_t = p_matrix
    t = 1
    while maximum([abs(P_t[1,s] - stat_dist[s]) for s in 1:n]) > epsilon
        t += 1
        P_t *= p_matrix
    end
    return t
end



p_matrix = [0.0 0.3 0.1 0.6;
            0.1 0.1 0.7 0.1;
            0.1 0.7 0.1 0.1;
            0.9 0.1 0.0 0.0]

epsilons = [0.1, 0.01, 0.001]

STEPS_32 = 32
STEPS_128 = 128

println("---[ ex13 ]---")

println("a) Stationary Distribution")
println(stationary_distribution(p_matrix))
println()

println("b) From 0 to 3 in ", STEPS_32, " steps")
p_matrix_32 = p_matrix ^ STEPS_32
display(p_matrix_32)
println()
println(p_matrix_32[1,4])
println()

println("c) From any to 3 in ", STEPS_128, " steps")
p_matrix_128 = p_matrix ^ STEPS_128
display(p_matrix_128)
println()
display(p_matrix_128[:,4])
println()
# println(sum(p_matrix_128[:,4])/4)
println((transpose(ones(Float64, 4) / 4) * p_matrix_128)[4])
println()

println("d) Min numbers of steps")
for eps in epsilons
    println("eps=", eps, ":\tsteps=", calculate_approximate(p_matrix, eps))
end

