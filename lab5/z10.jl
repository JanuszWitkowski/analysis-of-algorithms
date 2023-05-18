using Dates

global const RING_SIZE = 5

function isIllegal(config)
    count = 0

    if config[1] == config[RING_SIZE]
        count += 1
    end

    for i in 2:RING_SIZE
        if config[i - 1] != config[i]
            count += 1
        end
        if count == 2
            return true
        end
    end

    return false
end

function getConfigs(i, set)
    if i == RING_SIZE + 1
        return set
    else
        new_set = Set{Array{UInt8,1}}()

        for j in 0:RING_SIZE
            for config in set
                new_config = copy(config)
                new_config[i] = j
                push!(new_set, new_config)
            end
        end

        return getConfigs(i + 1, new_set)
    end
end

function getAllConfigs()
    timer = now()
    set = Set{Array{UInt8,1}}()
    push!(set, zeros(RING_SIZE))
    configs = getConfigs(1, set)
    println("Time of configs generation: ", canonicalize(Dates.CompoundPeriod(now() - timer)))
    return configs
end

function maxSteps()
    all_configs = getAllConfigs()
    println("Number of all configs: ", length(all_configs))

    timer = now()
    configs = Set{Array{UInt8,1}}(
        filter(config -> isIllegal(config), all_configs)
    )
    println("Number of illegal configs: ", length(all_configs))

    steps = 0
    println("Starting...")

    while !isempty(configs)
        new_configs = Set{Array{UInt8,1}}()

        for config in configs
            if config[1] == config[RING_SIZE]
                new_one = copy(config)
                new_one[1] = (new_one[1] + 1) % (RING_SIZE + 1)
                push!(new_configs, new_one)
            end

            for i in 2:RING_SIZE
                if config[i - 1] != config[i]
                    new_one = copy(config)
                    new_one[i] = new_one[i - 1]
                    push!(new_configs, new_one)
                end
            end
        end

        configs = Set{Array{UInt8,1}}(
            filter(config -> isIllegal(config), new_configs)
        )

        steps += 1
        println("Finished step ", steps)
    end

    println("Time of simulation: ", canonicalize(Dates.CompoundPeriod(now() - timer)))
    return steps
end

println("max_steps = ", maxSteps())
