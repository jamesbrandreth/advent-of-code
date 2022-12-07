import Base

function parseInstruction(s::String)
    _, n, _, from, _, to = split(s," ")
    n = parse(Int64, n)
    from = parse(Int64, from)
    to = parse(Int64, to)
    return n, from, to
end

struct Ship
    stacks::Vector{Vector{Char}}
end

function Base.show(io::IO, s::Ship)
    output = ""
    n_rows = maximum([length(stack) for stack in s.stacks])
    for i in n_rows:-1:1
        for stack in s.stacks
            c = (length(stack)<i) ? " " : stack[i]
            output=output*c
        end
        output=output*"\n"
    end
    print(output)
end

set_state_mode = true
open("input.txt", "r") do file
    lines = collect(eachline(file))
    state_end = findfirst(x->x=="", lines)

    # Set the state
    n_stacks = parse(Int64, strip(lines[state_end-1])[end])
    ship = Ship([Char[] for _ in 1:n_stacks])
    for line in lines[state_end-2:-1:1]
        for letter_index in findall(x->isletter(x), line)
            push!(ship.stacks[Int((letter_index+2)/4)], line[letter_index])
        end
    end

    println(ship)

    # Follow the instructions
    for line in lines[state_end+1:end]
        n, from, to = parseInstruction(String(line))
        for i in 1:n
            push!(ship.stacks[to], pop!(ship.stacks[from]))
        end
    end

    println(ship)
    println(join([pop!(stack) for stack in ship.stacks]))
end
