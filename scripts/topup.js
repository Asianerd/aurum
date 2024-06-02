function CCUpdate() {
    console.log("cc update");
}

function setAmount(i) {
    document.querySelector("#amount input").value = i;
}

function validateInputs() {
    var amount = document.querySelector("#amount input").value;

    var ccNumber = document.querySelector("#card #credit-card #cc-number").value;
    var ccExpYear = document.querySelector("#card #data #cc-exp-year").value;
    var ccExpMonth = document.querySelector("#card #data #cc-exp-month").value;
    var ccCVC = document.querySelector("#card #data #cc-cvc").value;

    let flag = true;
    [amount, ccNumber, ccExpYear, ccExpMonth, ccCVC].forEach((e) => {
        if (!(/^\d+$/.test(e))) {
            flag = false;
        }
    })

    document.querySelector("#confirmation").ariaLabel = flag ? 'enabled' : 'disabled';

    return flag;
}

validateInputs();

function submitTopup() {
    var amount = document.querySelector("#amount input").value;

    if (!validateInputs()) {
        return;
    }

    console.log(`${AURUM_BACKEND_ADDRESS}/wallet/top_up/${selectedWallet}/${amount}`);

    sendPostRequest(`${AURUM_BACKEND_ADDRESS}/wallet/top_up/${selectedWallet}/${amount}`, login_info(), (r) => {
        let response = parseResponse(r);

        console.log(response);

        fetchWallets();
    })
}

// #region wallets
var selectedWallet = 0;
var wallets = [];

var currencyFormatter = new Intl.NumberFormat('en-UK', {
    style: 'currency',
    currency: 'MYR'
})

fetchWallets();

function fetchWallets() {
    sendPostRequest(`${AURUM_BACKEND_ADDRESS}/wallet/get_wallets`, login_info(), (r) => {
        wallets = JSON.parse(parseResponse(r));

        let container = document.querySelector("#wallet-section #choices");
        container.innerHTML = '';
        wallets.forEach((e) => {
            container.innerHTML += `<h4 onclick='selectWallet(${e['id']})'>${e['name']}</h4>`;
        });

        selectWallet(selectedWallet);
        toggleWalletChoices();
    })
}

function selectWallet(id) {
    let target = null;
    wallets.forEach((e) => {
        if (e['id'] == id) {
            target = {...e};
        }
    });

    selectedWallet = id;
    
    document.querySelector("#wallet-select > h4").innerHTML = target['name'];
    document.querySelector("#wallet-section > h4").innerHTML = `with ${currencyFormatter.format(target['balance'])}`;

    toggleWalletChoices();
}

function toggleWalletChoices() {
    document.querySelector("#wallet-select").ariaLabel = document.querySelector("#wallet-select").ariaLabel == "open" ? "closed" : "open";
}
// #endregion
