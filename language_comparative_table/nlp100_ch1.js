var DATA = require('./data.js');

function nlp00(){
    var s = "stressed";
    return s.split("").reverse().join("");
    // 文字列長が10^4程度であれば以下のコードより速い
    // 文字列長が10^5を超える場合は以下のコードの方が速い
    // var rv = "";
    // var n = s.length;
    // for(var i=0; i<n; ++i){
    //     rv += s[n-1-i];
    // }
    // return rv;
}

function nlp01(){
    var s = "パタトクカシーー";
    var res = "";
    for (var i=0; i<s.length; i+=2){
        res += s[i];
    }
    return res;
}

function nlp02(){
    var s = ["パトカー", "タクシー"];
    var res = "";
    for (var i=0; i<s[0].length; ++i){
        for(var j=0; j<s.length; ++j){
            res += s[j][i];
        }
    }
    return res;
}
function nlp03(){
}
function nlp04(){
}
function nlp05(){
    return "";
}
function nlp06(){
    return "";
}
function nlp07(){
    return "";
}
function nlp08(){
    return "";
}
function nlp09(){
    return "";
}

function measureElapsedTime(func){
    var start = new Date().getTime();

    var res = func();

    var end = new Date().getTime();
    console.log(func.name);
    console.log("Elapsed Time: " + (end - start) + "[ms]");
    console.log(res);
}

function main(){
    measureElapsedTime(nlp00);
    measureElapsedTime(nlp01);
    measureElapsedTime(nlp02);
    measureElapsedTime(nlp03);
    measureElapsedTime(nlp04);
    measureElapsedTime(nlp05);
    measureElapsedTime(nlp06);
    measureElapsedTime(nlp07);
    measureElapsedTime(nlp08);
    measureElapsedTime(nlp09);
}
main();
