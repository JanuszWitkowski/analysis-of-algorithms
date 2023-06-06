function generate_P_matrix(graph, n)
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

function generate_M_matrix(graph, alpha)
    (n, _) = size(graph)
    p_matrix = generate_P_matrix(graph, n)
    j_matrix = ones(Float64, n, n)
    return (1 - alpha) * p_matrix + alpha * (1/n) * j_matrix
end
