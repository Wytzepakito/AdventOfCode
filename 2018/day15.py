import multiprocessing as mp
from os import getpid
import timeit
seq1 = [0, 3, 6]
seq2 = [1, 3, 2]
seq3 = [2, 1, 3]
seq4 = [1, 2, 3]
seq5 = [2, 3, 1]
seq6 = [3, 2, 1]
seq7 = [3, 1, 2]


seq8 = [6,3,15,13,1,0]
seq= seq1




def get_index(process_stack, current_number, start_index_in_master):
    highest_index = None
    if process_stack==[]:
        return None
    else:
        for i in range(len(process_stack), 0, -1):
            if process_stack[i-1]==current_number:
                highest_index = i
                break
        if highest_index is not None:
            return highest_index+start_index_in_master
        else:
            return None
    



start_time = timeit.default_timer()

if __name__ == '__main__':  
    for i in range(30000000):
        current_number = seq[-1:][0]
        current_stack = seq[:-1]
        highest_index = None
        processes = []
        increment = len(current_stack)/5
        master_stack = []
        for i in range(0,5):
            lower_bound = round(increment * (i))
            upper_bound = round(increment * (i+1))

            process_stack = current_stack[lower_bound:upper_bound]
            master_stack.append((process_stack, current_number, lower_bound))


        pool = mp.Pool(5)

        
        result = pool.starmap(get_index, master_stack)
        pool.close()
        pool.join()
        

            





        
        highest_indices = [res for res in result if res is not None]

            
            

        if len(highest_indices)>0:
            print(len(seq)-max(highest_indices))
            seq.append(len(seq)-max(highest_indices))
        else:
            print(0)
            seq.append(0)

    print(timeit.default_time()-start_time)

