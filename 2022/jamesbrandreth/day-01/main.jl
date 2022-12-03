
totals = zeros(Int64,1)
let i = 1
    open("input.txt", "r") do file
        for line in eachline(file)
            if line == ""
                i += 1
                append!(totals, 0)
            else
                totals[i] += parse(Int64, line)
            end
        end
    end
end

global biggest = 0
for n in totals
    if n > biggest
        global biggest = n
    end
end


print(biggest)
