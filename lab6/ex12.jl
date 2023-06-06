#!/usr/local/bin/julia
include("matrices.jl")
include("stationary.jl")

alphas = [0, 0.15, 0.5, 1]

graph = [ 1 0 0 0 0 0;
          0 0 1 0 1 0;
          1 0 0 0 0 0;
          0 1 0 0 1 0;
          0 0 0 1 0 0;
          0 0 1 0 0 0 ]

graph_separated = [ 1 0 0 0 0 0;
                    0 0 0 0 1 0;
                    1 0 0 0 0 0;
                    0 1 0 0 1 0;
                    0 0 0 1 0 0;
                    0 0 1 0 0 0 ]

println("---[ ex12 ]---")

println("ORIGINAL GRAPH")
for alpha in alphas
    println("alpha = ", alpha)
    m_matrix = generate_M_matrix(graph, alpha)
    println(stationary_distribution(m_matrix))
end
println()
println("SEPARATED GRAPH")
for alpha in alphas
    println("alpha = ", alpha)
    m_matrix = generate_M_matrix(graph_separated, alpha)
    println(stationary_distribution(m_matrix))
end

