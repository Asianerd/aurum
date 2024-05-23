var userCode;

var walletContainer = document.querySelector("#wallets");

var currencyFormatter = new Intl.NumberFormat('en-UK', {
    style: 'currency',
    currency: 'MYR'
})

var colour_themes = [
    "crimson",
    "#ec5d2a",
    "forestgreen",
    "dodgerblue",
    "blueviolet"
];

document.querySelector("#parent #username").innerHTML = username;

function toggleDurationSelection() {
    let e = document.querySelector("#selection #choices");
    e.ariaLabel = e.ariaLabel == "open" ? "closed" : "open";
}

function selectDuration(d) {
    document.querySelector("#history-title #selection > h5").innerHTML = {
        7: "Last 7 days",
        14: "Last 2 weeks",
        31: "Last month",
        365: "Last year",
        'all': "All time"
    }[d];

    toggleDurationSelection();
}

function toggleWalletCreate(b=null) {
    b = b === null ? false : b;
    document.querySelector("#overlay #wallet-creation").ariaLabel = b ? 'open' : 'closed';
}

// #region colours
// lazy to scope nicely
let _c = document.querySelectorAll("#colour-selection #colours");
_c.forEach((c) => {
    c.innerHTML = '';
    colour_themes.forEach((e, i) => {
        c.innerHTML += `<hr onclick="walletColourSelect(${i})" style="background:${e}">`;
    })
})

function toggleWalletColourThemes(e) {
    e.parentNode.querySelector("#colours").ariaLabel = e.parentNode.querySelector("#colours").ariaLabel == "open" ? "closed" : "open";
}

function toggleWalletDurationSelection() {
    document.querySelector("#wallet-creation #duration-selection").ariaLabel = document.querySelector("#wallet-creation #duration-selection").ariaLabel == "open" ? "closed" : "open";
}

function walletColourSelect(i) {
    document.querySelector("#overlay #wallet-creation").style.background = colour_themes[i];
    document.querySelector("#overlay #wallet-creation").dataset.colour = i;
}

function selectWalletDuration(t) {
    document.querySelector("#wallet-creation #duration-selection > h5").innerHTML = t;
    toggleWalletDurationSelection();
}

walletColourSelect(0);
// #endregion

// #region information updating
function updateInformation() {
    populateWallets();
    updateTotalBalance();
}

function populateWallets() {
    sendPostRequest(`${BACKEND_ADDRESS}/wallet/get_wallets`, login_info(), (r) => {
        let response = JSON.parse(parseResponse(r));

        walletContainer.innerHTML = '';

        response.forEach((e) => {
            walletContainer.innerHTML += `
<div class="wallet" ${e['id'] == 0 ? `id="main"` : ''} style="background:${colour_themes[e['colour']]}" onclick="toggleWalletInfo(${e['id']})">
    <h4>${e['name']}</h4>
    <h3>${currencyFormatter.format(e['balance'])}</h3>
</div>`;
        });
        walletContainer.innerHTML += `<div class="wallet" id="add" onclick="toggleWalletCreate(true);"><img src="/assets/add.png"></div>`;
    })
}

function updateTotalBalance() {
    sendPostRequest(`${BACKEND_ADDRESS}/wallet/get_total_balance`, login_info(), (r) => {
        let response = JSON.parse(parseResponse(r));

        document.querySelector("#balance > h3").innerHTML = currencyFormatter.format(response);
    })
}

updateInformation();
// #endregion

// #region wallet
function validateCreateWallet() {
    let flag = true;
    let name = document.querySelector("#wallet-creation > input[type=text]").value;
    let colour = document.querySelector("#wallet-creation").dataset.colour;

    if (!name) {
        flag = false;
    }
    if (colour == undefined) {
        flag = false;
    }

    document.querySelector("#wallet-creation #create-button").ariaLabel = flag ? "enabled" : "disabled";
    return flag;
}

validateCreateWallet();

function createWallet() {
    if (!validateCreateWallet()) {
        return;
    }

    let name = document.querySelector("#wallet-creation > input[type=text]").value;
    let colour = document.querySelector("#wallet-creation").dataset.colour;
    let limit = document.querySelector("#wallet-creation #limits > input[type=number]").value;
    let limit_type = document.querySelector("#wallet-creation #limits #duration-selection > h5:first-of-type").innerHTML;

    if ((limit == '') || (limit == 0)) {
        limit_type = "Unlimited";
        limit = 0;
    }

    let params = `${name}/${colour}/${limit_type}/${limit}`;

    console.log(params);

    sendPostRequest(`${BACKEND_ADDRESS}/wallet/create_wallet/${params}`, login_info(), () => {
        updateInformation();

        toggleWalletCreate(false);
    })
}
// #endregion

// #region logs
sendPostRequest(`${BACKEND_ADDRESS}/get_code`, login_info(), (r) => {
    let response = parseResponse(r);

    userCode = JSON.parse(response);

    updateLog();
})


function updateLog() {
    sendPostRequest(`${BACKEND_ADDRESS}/log/fetch_logs/${0}`, login_info(), (r) => {
        let response = JSON.parse(parseResponse(r));

        let container = document.querySelector("#history > table > tbody");
        container.innerHTML = '';
        response.forEach((e) => {
            // 4 diff types of transfers
            // topup, transfer to, transfer from, and transfer between wallets
            // let t = e.species;
            // let t;
            // if (e.species == "TopUp") {
            //     t = "TopUp";
            // } else {
            //     if (e.species.to == e.species.from) {
            //         t = "TransferBetween";
            //     } else {
            //         if (e.species.to_code == userCode) {
            //             t = "TransferFrom";
            //         } else {
            //             t = "TransferTo";
            //         }
            //     }
            // }

            let t = (e.species == "TopUp") ? "TopUp" : ((e.to == e.from) ? "TransferBetween" : (e.to_code == userCode ? "TransferFrom" : "TransferTo"));

            container.innerHTML += `
            <tr class="entry">
                <td><img id="icon" src="/assets/${{
                    "TopUp":"add",
                    "TransferTo":"outgoing",
                    "TransferFrom":"incoming",
                    "TransferBetween":"transfer"
                }[t]}.png"></td>
                <td>
                    <div>
                        <h4 id="target">${{
                            "TopUp": "Top Up",
                            "TransferTo": e.to_name,
                            "TransferFrom": e.from_name,
                            "TransferBetween": e.to_wallet_name
                        }[t]}</h4>
                        <h5 id="subscript">${formatDateTime(new Date(e.time * 1000))}</h5>
                    </div>
                </td>
                <td>
                    <div id="wallet-info">
                        <h4 id="amount">${{
                            "TopUp": "+",
                            "TransferTo": "-",
                            "TransferFrom": "+",
                            "TransferBetween": "+"
                        }[t]} ${currencyFormatter.format(Math.abs(e.amount))}</h4>
                        <span id="target">
                            <img src="/assets/curved_arrow.png">
                            <h5 id="subscript">${{
                                "TopUp": "to",
                                "TransferTo": "from",
                                "TransferFrom": "to",
                                "TransferBetween": "from"
                            }[t]} ${{
                                "TopUp": e.to_wallet_name,
                                "TransferTo": e.from_wallet_name,
                                "TransferFrom": e.to_wallet_name,
                                "TransferBetween": e.from_wallet_name
                            }[t]}</h5>
                        </span>
                    </div>
                </td>
            </tr>`;
        })
    });
}
// #endregion

// #region wallet updating
function toggleWalletInfo(w=null) {
    if (w === null) {
        document.querySelector("#overlay #wallet-info").ariaLabel = "closed";
    } else {
        console.log(w);
        document.querySelector("#overlay #wallet-info").ariaLabel = "open";
    }
}

function toggleWalletUpdateDurationSelection() {
    let d = document.querySelector("#overlay #wallet-info #duration-selection");
    d.ariaLabel = d.ariaLabel == 'open' ? 'closed' : 'open';
}
// #endregion
