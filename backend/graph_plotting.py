import matplotlib.pyplot as plt
import json
import os
import time
from multiprocessing import Process


def generate_graph(data):
    for x in data:
        fig, ax = plt.subplots()
        ax.set_ylabel(f"{x['name']} share sell price (USD)")
        ax.set_xticklabels([])
        
        _l = sorted(list(x['record'][1]))
        _min, _max = _l[0], _l[-1]
        _difference = abs(_min - _max) * 0.1
        plt.ylim(_min - _difference, _max + _difference)
        previous = -10
        for index, i in enumerate(x['record'][1]):
            if previous == -10:
                previous = i
                continue
            ax.bar(10 * index, i - previous, bottom=previous, width=5, color='#00ff00' if (i > previous) else '#ff0000')
            height = (i - previous)
            ax.bar(10 * index, height * 2, bottom=previous + (((i - previous)) / 2) - ((height * (1.75 if (i > previous) else 1.5)) / 2), width=1, color='#00ff00' if (i > previous) else '#ff0000')
            previous = i
        
        plt.savefig(f'graphs/{x["id"]}.png')
        plt.close()


tasks = []
def main():
    thread_count = 8
    child_count = 0
    
    data = []
    _, _, files = next(os.walk('./graph_data'))
    files.sort(key=lambda _x:int(_x.split('.')[0]))
    for x in files:
        with open(f'./graph_data/{x}', 'r') as file:
            data.append(json.load(file))
    print(f"{len(data)} file{'s' if len(data) > 1 else ''} loaded")

    child_count = len(files) // thread_count

    file_collection = []
    for x in range(thread_count):
        file_collection.append([])
        for y in range(child_count):
            file_collection[x].append(data[(x * child_count) + y])

    if sum([len(i) for i in file_collection]) != len(data):
        end = sum([len(i) for i in file_collection])
        target = len(data)
        for x in range(target - end):
            file_collection[-1].append(data[end + x])
    
    
    _start = time.time()
    for x in file_collection:
        tasks.append(Process(target=generate_graph, args=(x,)))
        tasks[-1].start()

    for x in tasks:
        x.join()
    print(f"Time elapsed : {time.time() - _start}")

if __name__ == '__main__':
    main()
