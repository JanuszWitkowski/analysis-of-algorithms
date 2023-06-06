ITER = 99999999

function stationary_distribution(matrix)
    (n, _) = size(matrix)
    return transpose(ones(Float64, n) / n) * (matrix ^ ITER)
end
