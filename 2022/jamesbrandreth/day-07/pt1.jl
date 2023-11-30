struct File
    name::String
    size::Int64
end

function size(file::File)
    return file.size
end

struct Directory
    name::String
    parent::Union{Directory, Missing}
    children::Vector{Union{File, Directory}}
end

function size(dir::Directory)
    return sum([size(child) for child in dir.children])
end

struct command
    name::String
    args::Vector{String}
    result::Vector{String}
end

function cd(args)
    body
end

global pwd::Directory
global file_system::Directory
f = open("input.txt", "r") do file
    read(file, String)
end
