function makeRange(s::String)
    first, last = split(s, "-")
    return parse(Int64, first):parse(Int64, last)
end

global total = 0
open("input.txt", "r") do file
    for line in eachline(file)
        range_1, range_2 = split(line,",")
        set_1 = Set(makeRange(String(range_1)))
        set_2 = Set(makeRange(String(range_2)))

        if ⊆(set_1, set_2) || ⊇(set_1, set_2)
            global total += 1
        end
    end
end

println(total)
