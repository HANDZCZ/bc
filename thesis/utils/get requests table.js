var [header, body, ..._] = document.querySelectorAll("div[role='table']>div");
header = [...header.querySelectorAll("&>div>div>div>div")].map(e => e.innerText);
body = [...body.querySelectorAll("&>div")].map(r => [...r.querySelectorAll("&>div")].map((e,i)=>{
    if(i==1){
        let a = e.querySelector("a");
        let r = e.querySelector(":not(a)");
        return `[${a.innerText}](${a.href})` + r.innerText.replace(a.innerText, "");
    }
    return e.innerText;
}));

var m = Math.max(...body.map(r=>parseInt(r[r.length-1].replace(" ", ""))));
header.push("Rozdíl oproti prvnímu " + header[header.length - 1].match(/\(\d+\)/)[0]);
body.forEach(r=>{
    var c = parseInt(r[r.length-1].replace(" ", ""));
    r.push((((m-c)/((m+c)/2))*100).toFixed(2) + "%");
});

var sep = header.map((_,i)=>{if(i<=1) return ":---"; else return "---:";}).join("|");
var res = `| ${header.join(" | ")} |\n| ${sep} |\n${body.map(e => e.join(" | ")).join("\n")}`;
console.log(res);
copy(res);