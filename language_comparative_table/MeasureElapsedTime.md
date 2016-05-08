# Python3, C++11, JavaScriptでの実行時間計測

C++とJSを勉強中なのですが、Python,C++,JSの３言語で実行時間計測をどのように書くのか調べてみました。サンプルとして$10^9$回足し算をする時間を計測します。

## Python3
まずは普段使っているPython。バージョンはPython 3.5.1。
``` python
import time

def DoSomething():
    a = 0
    for i in range(1000000000):
        a += 1

START_TIME = time.time()

DoSomething()

ELAPSED_TIME = time.time() - START_TIME
print('Elapsed Time:', ELAPSED_TIME*1000 ,'[ms]')
```
## C++11
バージョンはg++ (x86_64-win32-seh-rev0, Built by MinGW-W64 project) 5.1.0。
`g++ -std=c++0x`でコンパイルしました。（最適化オプションは付けていません。）
``` cpp
#include <iostream>
#include <chrono>

long DoSomething(){
    long a=0;
    for(int i = 0; i<1000000000; ++i){
        ++a;
    }
    return a;
}

int main(){
    std::chrono::system_clock::time_point  start, end;
    start = std::chrono::system_clock::now();

    DoSomething();

    end = std::chrono::system_clock::now();
    double elapsed = std::chrono::duration_cast<std::chrono::milliseconds>(end-start).count();
    std::cout << "Elapsed Time: " << elapsed << "[ms]" << std::endl;

}
```

## JavaScript
最近競プロで使っているのですが、まだ速いのか遅いのか分からない言語。環境はNode.js v4.4.4。
``` javascript
function DoSomething() {
    var a;
    for (var i = 0; i < 1000000000; ++i){
        ++a;
    }
}

var start = new Date().getTime();

DoSomething();

var end = new Date().getTime();
console.log("Elapsed Time: " + (end - start) + "[ms]");
```

## 結果
C++はPythonの40倍速い!  
JSもPythonの25倍速い!
|言語|実行時間[ms]|Python比|
|--|--:|--:|
|Python3|91966|1|
|C++|2216|0.0241|
|JS|3646|0.0396|

これは強い人がC++を使うのも全くよく分かります。
