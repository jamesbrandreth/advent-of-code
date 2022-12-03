
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

totals = sort(totals, rev=true)
x = sum(totals[begin:1:3])
print(x)
