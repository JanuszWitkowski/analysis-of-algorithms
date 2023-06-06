#!/usr/local/bin/julia
using LinearAlgebra
include("matrices.jl")
include("stationary.jl")

function calculate_data_for_plotting(graph, a)
    (n, _) = size(graph)
    m_matrix = generate_M_matrix(graph, a)
    stat_dist = stationary_distribution(m_matrix)
    ones_trans = transpose(ones(Float64, n) / n)
    M_t = m_matrix
    data = zeros(Float64, 25)
    for i in 1:25
        M_t *= m_matrix
        pi_k = ones_trans * M_t
        data[i] = norm(pi_k - stat_dist)
    end
    return data
end



alphas = [0, 0.25, 0.5, 0.75, 0.85, 1]

graph = [ 0 1 1 0 0;
          0 0 0 1 0;
          0 1 0 1 1;
          1 0 0 0 0;
          0 0 0 0 0 ]

data = zeros(Float64, 6, 25)

println("---[ ex14 ]---")

println("STATIONARY DISTRIBUTIONS")
for (i,alpha) in enumerate(alphas)
    println("alpha=", alpha)
    m_matrix = generate_M_matrix(graph, alpha)
    println(stationary_distribution(m_matrix))
    println()
    data[i,:] = calculate_data_for_plotting(graph, alpha)
end
println()
println("CONVERGENCE")
for i in 1:6
    println(join(data[i,:], ';'))
end
println()

