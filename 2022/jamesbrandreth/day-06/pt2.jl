

open("input.txt", "r") do file
    for line in eachline(file)
        line = collect(line)
        for i in 14:length(line)
            slice = line[i-13:i]
            if length(Set(slice)) == length(slice)
                println(i)
                break
            end
        end
    end
end
