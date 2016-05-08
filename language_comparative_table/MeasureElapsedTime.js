function DoSomething() {
    var a=0;
    for (var i = 0; i < 1000000; ++i){
        ++a;
    }
    return a;
}

function measureElapsedTime(func){
    var start = new Date().getTime();

    var res = func();

    var end = new Date().getTime();
    console.log("Elapsed Time: " + (end - start) + "[ms]");
    console.log(res);
}

measureElapsedTime(DoSomething);
