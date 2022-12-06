function priority(letter::Char)::Int64
    value = Int(letter)
    return value > 96 ? value-96 : value-38
end

global total = 0
open("input.txt", "r") do file
    for group in Iterators.partition(eachline(file),3)
        global total += priority(
            first(
                intersect(
                    Set(collect(group[1])),
                    Set(collect(group[2])),
                    Set(collect(group[3])),
                )
            )
        )
    end
end

println(total)
