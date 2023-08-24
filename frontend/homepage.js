var stockContainer = document.getElementsByClassName("stock-selection")[0];
var ip = "127.0.0.1:5000";

writeStocks();

async function writeStocks() {
    var response = await fetch(`http://${ip}/all`);
    var data = '';
    await response.json().then(function(result) {
        data = result;
    });
    data.forEach(element => {
        // <div class="stock" style="background-image:linear-gradient(#0000, #0000, #0000, #0000, #0000, #0009,#0009,#0007),
        // url('../backend/graphs/${element["id"]}.png') !important;">
        stockContainer.innerHTML += `
        <div class="stock" style="background-image:url('../backend/graphs/${element["id"]}.png') !important;">
            <div id="pricing">
                <div>
                    <h1>$${element["buy"]}</h1>
                    <h3>buy</h3>
                </div>
                <div>
                    <h1>$${element["sell"]}</h1>
                    <h3>sell</h3>
                </div>
                <div>
                    <h1>0</h1>
                    <h3>in possesion</h3>
                </div>
            </div>
            <div id="title">
                <div>
                    <img src="./assets/${element["growth"] == 0 ? "neutral" : Math.sign(element["growth"]) == 1 ? "profit" : "loss"}.png">
                    ${element["growth"] == 0 ?
                        '' :
                        `<p style="color:${Math.sign(element["growth"]) == 1 ? "#00ff00" : "#ff0000"} !important;">${Math.abs(element["growth"])}%</p>`
                    }
                </div>
                <h1 id="name">${element["name"]}</h1>
            </div>
        </div>
        `;
    });
}
