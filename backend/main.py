import time
import json
import random
import math
import sys
import os
import matplotlib.pyplot as plt


collection = []

class Stock:
    def __init__(self, _id, name, record, angle, fluctuation, buy_difference):
        self.id = _id
        
        self.name = name
        self.buy = record[0][0]
        self.sell = record[1][0]
        
        self.record = record
        self.previous_sell = self.record[1][1]
        
        self.angle = angle / 1000
        self.fluctuation = fluctuation / 1000
        self.buy_difference = buy_difference
        
        self.growth = 0
    
    def iterate(self):
        self.angle = abs(random.randint(int((90 - self.fluctuation) * 10000), int((90 + self.fluctuation) * 10000)) / 10000)
        self.growth = math.cos(math.radians(self.angle))
        
        self.previous_sell = self.sell
        self.sell *= (self.growth + 1)
        self.buy = (random.randint(9950, 10050) / 10000) * self.buy_difference * self.sell
        
        self.record[0].insert(0, self.buy)
        self.record[1].insert(0, self.sell)
        
        plt.clf()
        plt.style.use(['dark_background'])
        
        # fig, ax = plt.subplots()
        # ax.set_ylabel(f"{self.name} share sell price (USD)")
        # ax.set_xticklabels([])
        
        # _l = sorted(list(self.record[1]))
        # _min, _max = _l[0], _l[-1]
        # _difference = abs(_min - _max) * 0.1
        # plt.ylim(_min - _difference, _max + _difference)
        # previous = -10
        # for index, i in enumerate(self.record[1]):
        #     if previous == -10:
        #         previous = i
        #         continue
        #     ax.bar(10 * index, i - previous, bottom=previous, width=5, color='#00ff00' if (i > previous) else '#ff0000')
        #     height = (i - previous)
        #     ax.bar(10 * index, height * 2, bottom=previous + (((i - previous)) / 2) - ((height * (1.75 if (i > previous) else 1.5)) / 2), width=1, color='#00ff00' if (i > previous) else '#ff0000')
        #     previous = i
        
        # plt.savefig(f'graphs/{self.id}.png')
        # plt.close()

def age_stocks():
    collection = []
    
    print("start")
    data = []
    _, _, files = next(os.walk('./graph_data'))
    files.sort(key=lambda _x:int(_x.split('.')[0]))
    for x in files:
        with open(f'./graph_data/{x}', 'r') as file:
            data.append(json.load(file))
    print(f"{len(data)} file{'s' if len(data) > 1 else ''} loaded")
    
    for x in data:
        collection.append(Stock(
            x['id'],
            x['name'],
            x['record'],
            x['angle'],
            x['fluctuation'],
            x['buy_difference']
        ))
    print("data appended")
    
    _i = 1
    for x in collection:
        x.iterate()
        print(f"{_i} stocks iterated")
        _i += 1
    print("data iterated")
    
    with open('parsed_stocks.json', 'w') as file:
        json.dump([
            {
                "id":x.id,
                "name":x.name,
                "buy":round(x.buy, 2),
                "sell":round(x.sell, 2),
                "growth":round(x.growth * 100, 2)
            }
            for x in collection], file, indent=4)
    print("parsed data saved")
    
    for x in collection:
        with open(f'graph_data/{x.id}.json', 'w') as file:
            json.dump({
                "id":x.id,
                "name":x.name,
                "record":[i[0:100] for i in x.record],
                "angle":x.angle * 1000,
                "fluctuation":x.fluctuation * 1000,
                "buy_difference":x.buy_difference
            }, file, indent=4)
    print("data saved")

c = 1
if len(sys.argv) > 1:
    c = int(sys.argv[1])

i = 0
while c != 0:
    if c != -1:
        c -= 1
    with open('config.json', 'r') as file:
        config = json.load(file)
        if not config['iterate']:
            break
    print(f'Age : {i}')
    time.sleep(1)
    age_stocks()
    i += 1
    
