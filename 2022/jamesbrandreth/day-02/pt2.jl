global decode = Dict{String, Int64}(
    "A X" => 3, # rock, lose -> scissors -> 0+3=3
    "A Y" => 4, # rock, draw -> rock -> 3+1=4
    "A Z" => 8, # rock, win -> paper -> 6+2=8
    "B X" => 1, # paper, lose -> rock -> 0+1=1
    "B Y" => 5, # paper, draw -> paper -> 3+2=5
    "B Z" => 9, # paper, win -> scissors -> 6+3=9
    "C X" => 2, # scissors, lose -> paper -> 0+2=2
    "C Y" => 6, # scissors, draw -> scissors -> 3+3=6
    "C Z" => 7, # scissors, win -> rock -> 6+1=7
)

global score = 0

open("input.txt", "r") do file
    for line in eachline(file)
        global score += decode[line]
    end
end

println(score)
