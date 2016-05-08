#include <iostream>
#include <chrono>

long DoSomething(){
    long a=0;
    for(int i = 0; i<1000000000; ++i){
        ++a;
        if(a%2 == 0){
            a /= 2;
        }else{
            a = a * 3 + 1;
        }
    }
    return a;
}


template<class F>
void MeasureElapsedTime(F func){
    std::chrono::system_clock::time_point  start, end;
    start = std::chrono::system_clock::now();

    auto res = func();

    end = std::chrono::system_clock::now();
    double elapsed = std::chrono::duration_cast<std::chrono::milliseconds>(end-start).count();
    std::cout << "Elapsed Time: " << elapsed << "[ms]" << std::endl;
    std::cout << res << std::endl;
}

int main(){
    MeasureElapsedTime(DoSomething);
}
