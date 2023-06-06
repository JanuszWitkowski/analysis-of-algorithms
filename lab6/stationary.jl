ITER = 123456789

function stationary_distribution(matrix)
    (n, _) = size(matrix)
    starting = ones(Float64, n) / n
    return transpose(starting) * (matrix ^ ITER)
    # display(matrix)
    # display(starting)
    # res = (starting \ matrix) / n
    # display(res)
    # return res
end
