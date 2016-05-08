def DoSomething():
    a = 0
    for i in range(100000):
        a += 1
    return a


def measure_elapsed_time(func):
    import time
    START_TIME = time.time()

    res = func()

    ELAPSED_TIME = time.time() - START_TIME

    print('Elapsed Time:', ELAPSED_TIME * 1000, '[ms]')
    print(res)

if __name__ == '__main__':
    measure_elapsed_time(DoSomething)
