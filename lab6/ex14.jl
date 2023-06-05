using LinearAlgebra

function P_matrix(graph, n)
    p_matrix = zeros(Float64, n, n)
    for i in 1:n
        row_sum = sum(graph[i,:])
        if row_sum == 0
            for j in 1:n
                p_matrix[i,j] = 1/n
            end
        else
            for j in 1:n
                p_matrix[i,j] = graph[i,j]/row_sum
            end
        end
    end
    return p_matrix
end

function M_matrix(graph, a)
    n = length(graph[1,:])
    p_matrix = P_matrix(graph, n)
    j_matrix = ones(Float64, n, n)
    
    return (1-a) * p_matrix + a * (1/n) * j_matrix
end

function Stationary_dist(graph, a)
    n = length(graph[1,:])
    m_matrix = M_matrix(graph, a)

    return transpose(ones(Float64, n) / n) * (m_matrix ^ 99999999)
end

function Convergence(graph, a)
    n = length(graph[1,:])
    m_matrix = M_matrix(graph, a)
    pi_vector = transpose(ones(Float64, n) / n) * (m_matrix ^ 99999999)
    pi_k = ones(Float64, n) / n
    m_t = m_matrix
    data = zeros(Float64, 25)
    for i in 1:25
        m_t *= m_matrix
        pi_k = transpose(ones(Float64, n) / n) * m_t
        data[i] = norm(pi_k - pi_vector)
    end
    return data
end

alphas = [0 0.25 0.5 0.75 0.85 1]

graph = [0 1 1 0 0;
         0 0 0 1 0;
         0 1 0 1 1;
         1 0 0 0 0;
         0 0 0 0 0]

data = zeros(Float64, 6, 25)

for (i,a) in enumerate(alphas)
    println("alpha = $a")
    println(Stationary_dist(graph, a))
    data[i,:] = Convergence(graph, a)
end

for i in 1:6
    println(join(data[i,:], ','))
end
println()

