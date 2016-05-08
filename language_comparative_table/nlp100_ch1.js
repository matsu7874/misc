var DATA = require("./data.js");

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
    var s = "Now I need a drink, alcoholic of course, " +
    "after the heavy lectures involving quantum mechanics.";
    var words = s.replace(/\./g,"").replace(/,/g,"").split(" ");
    // 文字列中の全ての一致パターンを置換するときは正規表現のglobalマッチ(g)
    // .は正規表現ではエスケープが必要
    var res = [];
    for(var i=0; i < words.length; ++i){
        res.push(words[i].length);
    }
    return res;
}
function nlp04(){
    var s = "Hi He Lied Because Boron Could Not Oxidize Fluorine. " +
    "New Nations Might Also Sign Peace Security Clause. Arthur King Can.";
    var words = s.replace(/\./g,"").replace(/,/g,"").split(" ");
    var res = {};
    var shorts = [1, 5, 6, 7, 8, 9, 15, 16, 19];
    for(var i=0; i<words.length; ++i){
        res[words[i].substr(0,(shorts.indexOf(i+1)===-1)+1)] = i+1;
    }
    return res;
}

function nGram(seq, n){
    //seqは文字列 or Array。どちらも同様の挙動をするsliceがある。
    var res = [];
    for(var i=0; i<seq.length-n+1; ++i){
        res.push(seq.slice(i,i+n));
    }
    return res;
}

function nlp05(){
    var s = "I am an NLPer";
    var res = {};
    res["Character-N-gram"] = nGram(s,2);
    var words = s.replace(/\./g,"").replace(/,/g,"").split(" ");
    res["Word-N-gram"] = nGram(words,2);
    return res;
}

function set(a){
    // O(|a| * log|a|)
    if (a.length === 0){
        return [];
    }
    a.sort(function(a, b){
        if( a < b ) return -1;
        if( a > b ) return 1;
        return 0;
    });
    var res = [a[0]];
    for(var i=1; i<a.length; ++i){
        if (a[i-1] < a[i]){
            res.push(a[i]);
        }
    }
    return res;
}

function union(a, b){
    // ((|a|+|b|) * log(|a|+|b|))
    var tmp = [];
    a = set(a);
    b = set(b);
    for(var i=0; i<a.length; ++i){
        tmp.push(a[i]);
    }
    for(var i=0; i<b.length; ++i){
        tmp.push(b[i]);
    }
    if(tmp.length === 0){
        return [];
    }
    tmp.sort(function(a, b){
        if( a < b ) return -1;
        if( a > b ) return 1;
        return 0;
    });
    var res = [tmp[0]];
    for(var i=1; i<tmp.length; ++i){
        if (tmp[i-1] < tmp[i]){
            res.push(tmp[i]);
        }
    }
    return res;
}

function nlp06(){
    var s = "paraparaparadise";
    var t =  "paragraph";
    var X = nGram(s, 2);
    var Y = nGram(t, 2);

    return union(X,Y);
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