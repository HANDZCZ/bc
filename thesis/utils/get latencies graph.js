var step = 20;
var max_offset = 40;
// -----
var [header, body, ..._] = document.querySelectorAll("div[role='table']>div");
body = [...body.querySelectorAll("&>div")].map(r => [...r.querySelectorAll("&>div")].map(e=> e.innerText));

label = [...header.querySelectorAll("&>div>div>div>div")].slice(2).map(e => `'Concurency ${e.innerText.match(/(?:\((\d+)\))/)[1]}'`).join(", ");
frameworks = body.map(a => a.splice(1,1)[0]);
data = body.map(a => a.splice(1).map(e => e.replace(" ", "")));

var max = Math.max(...data.map(a => a[2]).map(d => parseFloat(d.replace("ms", ""))));

var res = `import plotly.graph_objects as go

x_label=[${label}]

fig = go.Figure(data=[
    ${frameworks.map((f,i) => `go.Bar(name='${f}', x=x_label, y=[${data[i].map(d => d.replace("ms", "")).join(", ")}], text=[${data[i].map(d => `'${d}'`).join(", ")}])`).join(",\n    ")}
])
fig.update_yaxes(
    tickmode='linear',
    tick0=0,
    dtick=${step},
    ticksuffix='ms',
    range=[0, ${Math.ceil(max + max_offset)}],
    showline=True,
    linewidth=2,
    linecolor='rgba(38,38,38,0.25)',
    gridcolor='rgba(38,38,38,0.15)'
)
fig.update_xaxes(
    showline=True,
    linewidth=2,
    linecolor='rgba(38,38,38,0.25)'
)
fig.update_traces(
    textangle=90,
    constraintext='inside',
    textfont_size=22,
    textposition='outside'
)
fig.update_layout(
    font_size=24,
    bargroupgap=0.2,
    barmode='group',
    legend=dict(
        orientation='h',
        yanchor='bottom',
        y=1.0,
        xanchor='center',
        x=0.5
    ),
    margin=dict(l=0, r=0, t=0, b=0),
    paper_bgcolor='rgba(0,0,0,0)',
    plot_bgcolor='rgba(0,0,0,0)',
    width=2000,
    height=1000
)`;
console.log(res);
copy(res);