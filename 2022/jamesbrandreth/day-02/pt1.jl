global decode = Dict{String, String}(
    "A" => "rock",
    "X" => "rock",
    "B" => "paper",
    "Y" => "paper",
    "C" => "scissors",
    "Z" => "scissors",
)

global winner = Dict{Set, String}(
    Set(["rock", "paper"]) => "paper",
    Set(["paper", "scissors"]) => "scissors",
    Set(["scissors", "rock"]) => "rock",
)

value = Dict{String, Int64}(
    "rock" => 1,
    "paper" => 2,
    "scissors" => 3,
)

function beats(play::String, opps::String)::Int64
    if play == opps
        return 3
    end
    return winner[Set([play,opps])]==play ? 6 : 0
end

global score = 0

open("input.txt", "r") do file
    for line in eachline(file)
        opps, play = split(line, " ")
        opps = decode[opps]
        play = decode[play]
        global score += (value[play] + beats(play, opps))
    end
end

println(score)
