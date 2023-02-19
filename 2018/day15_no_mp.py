seq1 = [0, 3, 6]
seq2 = [1, 3, 2]
seq3 = [2, 1, 3]
seq4 = [1, 2, 3]
seq5 = [2, 3, 1]
seq6 = [3, 2, 1]
seq7 = [3, 1, 2]
seq8 = [6, 3, 15, 13, 1, 0]

seq = seq8
lastturn = 30000000
spoken = {}

def add_to_dict(key, val):
    key = str(key)
    if key not in spoken:
        spoken[key] = {"recent": val, "old": None}
    else:
        spoken[key]["old"] = spoken[key]["recent"]
        spoken[key]["recent"] = val
    spoken["last"] = key

count = 1
for i in range(len(seq)):
    add_to_dict(seq[count-1], count)
    count+=1


while count <= lastturn:
    if spoken[spoken["last"]]["old"] is None:
        add_to_dict(0, count)
    else:
        diff = spoken[spoken["last"]]["recent"] - spoken[spoken["last"]]["old"]
        add_to_dict(diff, count)
    count += 1

print(spoken["last"])
    
