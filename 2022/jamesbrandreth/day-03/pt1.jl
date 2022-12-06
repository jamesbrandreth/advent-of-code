function priority(letter::Char)::Int64
    value = Int(letter)
    return value > 96 ? value-96 : value-38
end

global total = 0
open("input.txt", "r") do file
    for line in eachline(file)
        chars = collect(line)
        compartment_size = Int(length(chars)/2)
        compartment_1 = chars[1:compartment_size]
        compartment_2 = chars[compartment_size+1:end]
        global total += priority(first(intersect(compartment_1, compartment_2)))
    end
end

println(total)
