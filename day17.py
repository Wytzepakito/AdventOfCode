string = """#####..#
#..###.#
###.....
.#.#.#..
##.#..#.
######..
.##..###
###.####"""

string2 = """.#.
..#
###"""


active_nodes = []

for y_index,line in enumerate(string2.split("\n")):
    for x_index,letter in enumerate(line):
        if letter=="#":
            active_nodes.append((x_index-1, y_index-1, 0, 0))

print(len(active_nodes))
def get_neighbours(node):
    new_nodes = []
    #8
    w_range = [-1, 0, 1]
    new_nodes.append((node[0], node[1], node[2], node[3]-1))
    new_nodes.append((node[0], node[1], node[2], node[3]+1))
    for w in w_range:
        new_nodes.append((node[0]-1, node[1], node[2], node[3]+w))
        new_nodes.append((node[0]-1, node[1]-1, node[2], node[3]+w))
        new_nodes.append((node[0], node[1]-1, node[2], node[3]+w))
        new_nodes.append((node[0]+1, node[1]-1, node[2], node[3]+w))
        new_nodes.append((node[0]+1, node[1], node[2], node[3]+w))
        new_nodes.append((node[0]+1, node[1]+1, node[2], node[3]+w))
        new_nodes.append((node[0], node[1]+1, node[2], node[3]+w))
        new_nodes.append((node[0]-1, node[1]+1, node[2], node[3]+w))

        new_nodes.append((node[0]-1, node[1], node[2]+1, node[3]+w))
        new_nodes.append((node[0]-1, node[1]-1, node[2]+1, node[3]+w))
        new_nodes.append((node[0], node[1]-1, node[2]+1, node[3]+w))
        new_nodes.append((node[0]+1, node[1]-1, node[2]+1, node[3]+w))
        new_nodes.append((node[0]+1, node[1], node[2]+1, node[3]+w))
        new_nodes.append((node[0]+1, node[1]+1, node[2]+1, node[3]+w))
        new_nodes.append((node[0], node[1]+1, node[2]+1, node[3]+w))
        new_nodes.append((node[0]-1, node[1]+1, node[2]+1, node[3]+w))
        new_nodes.append((node[0], node[1], node[2]+1, node[3]+w))

        new_nodes.append((node[0]-1, node[1], node[2]-1, node[3]+w))
        new_nodes.append((node[0]-1, node[1]-1, node[2]-1, node[3]+w))
        new_nodes.append((node[0], node[1]-1, node[2]-1, node[3]+w))
        new_nodes.append((node[0]+1, node[1]-1, node[2]-1, node[3]+w))
        new_nodes.append((node[0]+1, node[1], node[2]-1, node[3]+w))
        new_nodes.append((node[0]+1, node[1]+1, node[2]-1, node[3]+w))
        new_nodes.append((node[0], node[1]+1, node[2]-1, node[3]+w))
        new_nodes.append((node[0]-1, node[1]+1, node[2]-1, node[3]+w))
        new_nodes.append((node[0], node[1], node[2]-1, node[3]+w))
    return new_nodes






node =get_neighbours((0,0,0,0))
    
cycles = 6


for i in range(cycles):
    print(i)
    possible_new_nodes = []
    for node in active_nodes:
        possible_new_nodes.extend(get_neighbours(node))
        
    # remove active nodes from candidate inactive nodes
    for active_node in active_nodes:
        if active_node in possible_new_nodes:
            possible_new_nodes.remove(active_node)

    possible_new_nodes = list(set(possible_new_nodes))

            
    new_active_nodes = []

    # Check all inactive nodes
    for node in possible_new_nodes:
        count = 0
        neighbours = get_neighbours(node)
        for activator_node in neighbours:
            if activator_node in active_nodes:
                count += 1
        if count==3:
            new_active_nodes.append(node)
    for node in active_nodes:
        count = 0
        neighbours = get_neighbours(node)
        for activator_node in neighbours:
            if activator_node in active_nodes:
                count += 1
        if count==3 or count==2:
            new_active_nodes.append(node)
    active_nodes = list(set(new_active_nodes))
        
                
    
    

