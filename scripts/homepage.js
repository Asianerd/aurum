var username = fetchLocalStorage("aurum_username");
var password = fetchLocalStorage("aurum_password");

if ((username === null) || (password === null)) {
    window.location.href = "./login.html";
}

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

function login_info() {
    return JSON.stringify({
        "username":username,
        "password":password
    });
}

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
function toggleWalletColourThemes() {
    document.querySelector("#wallet-creation #colour-selection #colours").ariaLabel = document.querySelector("#wallet-creation #colour-selection #colours").ariaLabel == "open" ? "closed" : "open";
}

function walletColourSelect(i) {
    document.querySelector("#overlay #wallet-creation").style.background = colour_themes[i];
    document.querySelector("#overlay #wallet-creation").dataset.colour = i;
}

// lazy to scope nicely
let _c = document.querySelector("#overlay #wallet-creation #colours");
_c.innerHTML = '';
colour_themes.forEach((e, i) => {
    _c.innerHTML += `<hr onclick="walletColourSelect(${i})" style="background:${e}">`;
})
// #endregion

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
<div class="wallet" ${e['id'] == 0 ? `id="main"` : ''} style="background:${colour_themes[e['colour']]}">
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
