function DoSomething() {
    var a;
    for (var i = 0; i < 1000000000; ++i)
        ++a;
}

var start = new Date().getTime();

DoSomething();
var end = new Date().getTime();
console.log("Elapsed Time: " + (end - start) + "[ms]");
