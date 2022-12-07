input = readlines("data/day07.data")
test = readlines("data/day07.test")

struct Inode 
    name::String
    parent::Union{Inode, Nothing}
    children::Dict{String, Inode}
    size::Int
    isfile::Bool
end

function parse_lines(lines)
    root = Inode("root", nothing, Dict(), 0, false)
    curr = root

    for line in lines
        cmds = split(line, " ")
        if cmds[1] == "\$"
            if cmds[2] == "cd"
                if cmds[3] == ".."
                    curr = curr.parent
                elseif cmds[3] == "/"
                    curr = root
                else
                    curr = curr.children[cmds[3]]
                end
            end
        elseif cmds[1] == "dir"
            if !haskey(curr.children, cmds[2])
                curr.children[cmds[2]] = Inode(cmds[2], curr, Dict(), 0, false)
            end
        else
            if !haskey(curr.children, cmds[2])
                curr.children[cmds[2]] = Inode(cmds[2], curr, Dict(), parse(Int, cmds[1]), true)
            end
        end
    end
    root
end

function folder_sizes(folder)
    folder_size = 0
    sizes = []
    for (name, child) in folder.children
        if child.isfile
            folder_size += child.size
        else
            fs, sz = folder_sizes(child)
            folder_size += fs
            append!(sizes, sz)
        end
    end
    push!(sizes, folder_size)
    folder_size, sizes
end

# Test
filetree = parse_lines(test)

# Real 
filetree = parse_lines(input)

rootsize, fsizes = folder_sizes(filetree)

# Solve a
sum(filter(<=(100000), fsizes))

# Solve b
unused = 70_000_000 - rootsize
to_free = 30_000_000 - unused

minimum(filter(>=(to_free), fsizes))