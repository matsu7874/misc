import time

def DoSomething():
    a = 0
    for i in range(1000000000):
        a += 1

START_TIME = time.time()

DoSomething()

ELAPSED_TIME = time.time() - START_TIME
print('Elapsed Time:', ELAPSED_TIME*1000 ,'[ms]')
