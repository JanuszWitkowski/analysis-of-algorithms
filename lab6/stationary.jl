ITER = 99999999

function stationary_distribution(matrix)
    n = length(matrix[1,:])
    return transpose(ones(Float64, n) / n) * (matrix ^ ITER)
end
