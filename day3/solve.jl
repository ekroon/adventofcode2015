module Solve

# Pkg.add("Match")
using Base.Test
using Match
using Lazy

function to_coord(c::Char)::Complex{Int}
    @match c begin
        '^' => 0 + im
        'v' => 0 - im
        '>' => 1 + 0im
        '<' => -1 + 0im
        _ => AssertionError("Should not happen")
    end
end

function gift_locations(indexes, instructions)
    map(i -> instructions[i], indexes) |>
    v -> map(to_coord, v) |>
    v -> accumulate(+, v)
end

@test gift_locations([1,2,3], ['^', '^', '^']) == [1im, 2im, 3im]

function combine_gifts(input)
    input |>
    frequencies |>
    values |>
    v -> count(_ -> true, v) + 1
end

@test combine_gifts([1im, 2im, 3im, 2im, 1im]) == 4

function solve1(input)
    instructions = input |> strip |> collect
    santa = (1:endof(instructions)) |> collect
    result = gift_locations(santa, instructions) |> combine_gifts
    result
end

function solve2(input)
    instructions = input |> strip |> collect
    santa = (1:2:endof(instructions)) |> collect
    robo_santa = (2:2:endof(instructions)) |> collect
    result = vcat(gift_locations(santa, instructions),
        gift_locations(robo_santa, instructions)) |>
        combine_gifts
        result - 1 # double first house
end

function main()
    if (typeof(STDIN) === Base.TTY)
            return println("Use input file as STDIN")
    end
    input = readstring(STDIN)
    println(@sprintf("part1: %s", solve1(input)))
    println(@sprintf("part2: %s", solve2(input)))
end

@time main()

end
