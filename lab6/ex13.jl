function Stationary_dist(p_matrix)
    n = length(p_matrix[1,:])
    return transpose(ones(Float64, n) / n) * (p_matrix ^ 99999999)
end

function Approx(p_matrix, epsilon)
    n = length(p_matrix[1,:])
    pi_vector = transpose(ones(Float64, n) / n) * (p_matrix ^ 99999999)
    pi_approx = [0 0.3 0.1 0.6]
    p_t = p_matrix
    t = 1
    while maximum([abs(p_t[1,i] - pi_vector[i]) for i in 1:n]) > epsilon
        t += 1
        p_t *= p_matrix
    end
    return t
end

p_matrix = [0.0 0.3 0.1 0.6;
            0.1 0.1 0.7 0.1;
            0.1 0.7 0.1 0.1;
            0.9 0.1 0.0 0.0]

epsilons = [0.1 0.01 0.001]

println("Rozkład stacjonarny:")
println(Stationary_dist(p_matrix))

println("\nPrawdopodobieństwo stanu 3 po 32 krokach zaczynając w 0:")
display(p_matrix ^ 32)
println()
println((p_matrix ^ 32)[1,4])

println("\nPrawdopodobieństwo stanu 3 po 128 krokach zaczynając w dowolnym:")
display(p_matrix ^ 128)
println()
println(sum((p_matrix ^ 128)[:,4])/4)

println("\nLiczba kroków:")
for epsilon in epsilons
    print("epsilon = $epsilon: ")
    println(Approx(p_matrix, epsilon))
end

