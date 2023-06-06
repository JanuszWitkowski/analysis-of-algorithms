ITER = 123456789

function stationary_distribution(matrix)
    (n, _) = size(matrix)
    return transpose(ones(Float64, n) / n) * (matrix ^ ITER)
end
