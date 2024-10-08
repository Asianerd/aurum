document.querySelector("#user-data #username").innerHTML = username;
sendPostRequest(`${AURUM_BACKEND_ADDRESS}/get_code`, login_info(), (r) => {
    let response = JSON.parse(parseResponse(r));

    document.querySelector("#overlay #self-qr > div").appendChild(QRCode({
        msg:response,
        dim:512,
        pal:["#7f7cff", "#000"],
        // pal:["#000", "#fff"],
        ecl:"H",
        vrb:1
    }));

    document.querySelector("#overlay #self-qr #title").innerHTML = `${username}'s qr code`;

    document.querySelector("#user-data #code").innerHTML = formatUserCode(response);
})

var selectedUser = null;
var selectedUserData = ['username', 'code']; // for transfer results
var selectedWallet = 0;
var selectedWalletTransfer = 0;
var surpassedLimit = false;

var users = [];
var wallets = [];

function selectUser(e, user_id, username, code) {
    let header = document.querySelector("#destination");
    header.ariaLabel = header.ariaLabel == "open" ? "closed" : "open";

    e.ariaLabel = e.ariaLabel == "selected" ? "" : "selected";

    if (header.ariaLabel == "open") {
        selectedUser = null;
    } else {
        selectedUser = user_id;
        selectedUserData = [username, code];
        console.log(selectedUserData);
    }

    validateTransfer();
}

function validateTransfer() {
    let flag = true;

    if (document.querySelector("#destination-select").ariaLabel == 'user') {
        if (selectedUser == null) {
            flag = false;
        }
        let amount = document.querySelector("#transfer-amount input").value;
        if ((amount == '') || (amount == 0)) {
            flag = false;
        }
        if (surpassedLimit) {
            flag = false;
        }
    } else {
        let amount = document.querySelector("#transfer-amount input").value;
        if ((amount == '') || (amount == 0)) {
            flag = false;
        }
        
        if (selectedWallet == selectedWalletTransfer) {
            flag = false;
        }
    }

    document.querySelector("#confirmation").ariaLabel = flag ? "enabled" : "disabled";

    return flag;
}

validateTransfer();

function submitTransfer() {
    if (!validateTransfer()) {
        return;
    }

    if (document.querySelector("#destination-select").ariaLabel == 'user') {
        let amount = document.querySelector("#transfer-amount input").value;

        // /<from_wallet>/<to_user>/<to_wallet>/<amount>
        var params = `${selectedWallet}/${selectedUser}/0/${amount}`;
        sendPostRequest(`${AURUM_BACKEND_ADDRESS}/wallet/transfer_balance/${params}`, login_info(), (r) => {
            let response = JSON.parse(parseResponse(r));

            let state = response['Ok'];

            if (state == "Success") {
                displayTransferResult("Success!", `${currencyFormatter.format(amount)} successfully transferred to ${selectedUserData[0]} (#${selectedUserData[1].slice(0, 4)}-${selectedUserData[1].slice(4, 8)})`);
            } else {
                let d = {
                    "InsufficientAmount":"Insufficient funds",
                    "ReachedLimit":"Wallet limit exceeded!"
                }[state];
                displayTransferResult(
                    "Transfer failed!",
                    d == undefined ? "idk why lmao" : d
                )
            }

            fetchWallets();
        })
    } else {
        let amount = document.querySelector("#transfer-amount input").value;
        // /<from_wallet>/<to_wallet>/<amount>
        var params = `${selectedWallet}/${selectedWalletTransfer}/${amount}`;

        let result = [];

        wallets.forEach((e) => {
            if (e['id'] == selectedWallet) { result[0] = {...e}; }
        });
        wallets.forEach((e) => {
            if (e['id'] == selectedWalletTransfer) { result[1] = {...e}; }
        });

        sendPostRequest(`${AURUM_BACKEND_ADDRESS}/wallet/transfer_between_wallets/${params}`, login_info(), (r) => {
            let response = JSON.parse(parseResponse(r));

            let state = response['Ok'];

            if (state == "Success") {
                displayTransferResult("Success!", `${currencyFormatter.format(amount)} successfully transferred from ${result[0]['name']} to ${result[1]['name']}`);
            } else {
                let d = {
                    "InsufficientAmount":"Insufficient funds",
                    "ReachedLimit":"Wallet limit exceeded!"
                }[state];
                displayTransferResult(
                    "Transfer failed!",
                    d == undefined ? "idk why lmao" : d
                )
            }

            fetchWallets();
        })
    }
}

function displayTransferResult(header, description) {
    document.querySelector("#overlay #transfer-result #header").innerHTML = header;
    document.querySelector("#overlay #transfer-result #description").innerHTML = description;
    document.querySelector("#overlay #transfer-result").ariaLabel = 'open';
}

// #region wallets
fetchWallets();

function fetchWallets() {
    sendPostRequest(`${AURUM_BACKEND_ADDRESS}/wallet/get_wallets`, login_info(), (r) => {
        wallets = JSON.parse(parseResponse(r));

        let container = document.querySelector("#wallet-section #choices");
        container.innerHTML = '';
        wallets.forEach((e) => {
            container.innerHTML += `<h4 onclick='selectWallet(${e['id']})'>${e['name']}</h4>`;
        });

        selectWallet(selectedWallet); // opens wallet by default
        toggleWalletChoices(); // closes it immediately
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

    sendPostRequest(`${AURUM_BACKEND_ADDRESS}/wallet/get_limit/${selectedWallet}`, login_info(), (r) => {
        let response = JSON.parse(parseResponse(r));

        let available = response[1] - response[0];
        surpassedLimit = (response[2] != 'unlimited') && (available <= 0);

        validateTransfer();

        document.querySelector("#limit-disclaimer").innerHTML = surpassedLimit ? `${currencyFormatter.format(response[1])} ${response[2]} limit reached!` : (response[2] != "unlimited" ? `${currencyFormatter.format(available)} until ${response[2]} limit` : "");
        document.querySelector("#limit-disclaimer").ariaLabel = surpassedLimit ? 'surpassed' : '';
    })
}

function toggleWalletChoices() {
    document.querySelector("#user-data #wallet-select").ariaLabel = document.querySelector("#user-data #wallet-select").ariaLabel == "open" ? "closed" : "open";
}
// #endregion

// #region inter-account transfer (between wallets)
function selectDestinationType(t) {
    if (t == "user") {
        document.querySelector("#destination-select #choices #wallet").ariaLabel = '';
        document.querySelector("#destination-select #choices #user").ariaLabel = 'selected';
        document.querySelector("#destination-select").ariaLabel = 'user';
    } else {
        document.querySelector("#destination-select #choices #wallet").ariaLabel = 'selected';
        document.querySelector("#destination-select #choices #user").ariaLabel = '';
        document.querySelector("#destination-select").ariaLabel = 'wallet';
    }

    validateTransfer();
}

fetchTransferWallets();

function fetchTransferWallets() {
    sendPostRequest(`${AURUM_BACKEND_ADDRESS}/wallet/get_wallets`, login_info(), (r) => {
        wallets = JSON.parse(parseResponse(r));

        let container = document.querySelector("#destination-select #wallet-section #choices");
        container.innerHTML = '';
        wallets.forEach((e) => {
            container.innerHTML += `<h4 onclick='selectWalletTransfer(${e['id']})'>${e['name']}</h4>`;
        });

        selectWalletTransfer(selectedWalletTransfer); // opens wallet by default
        toggleWalletTransferChoices(); // closes it immediately
    })
}

function selectWalletTransfer(id) {
    let target = null;
    wallets.forEach((e) => {
        if (e['id'] == id) {
            target = {...e};
        }
    });

    selectedWalletTransfer = id;
    
    document.querySelector("#destination-select #wallet-select > h4").innerHTML = target['name'];
    document.querySelector("#destination-select #wallet-section > h4").innerHTML = `with ${currencyFormatter.format(target['balance'])}`;

    toggleWalletTransferChoices();
    validateTransfer();
}

function toggleWalletTransferChoices() {
    let d = document.querySelector("#destination-select #wallet-select");
    d.ariaLabel = d.ariaLabel == "open" ? "closed" : "open";
}
// #endregion

queryUser();

function queryUser() {
    let container = document.querySelector("#query-result");
    if (!document.querySelector("#query-input input").value) {
        container.innerHTML = '';
        return;
    }

    sendPostRequest(`${AURUM_BACKEND_ADDRESS}/query_users/${encodeURIComponent(document.querySelector("#query-input input").value)}`, login_info(), (r) => {
        let response = JSON.parse(parseResponse(r));
        container.innerHTML = '';

        users = response;

        // show maybe only every 10 users
        // < page 1/10 >
        // ^page selection kind of thing

        users.forEach((e) => {
            container.innerHTML += `<div class="user" onclick="selectUser(this, ${e[0]}, '${e[2]}', '${e[1]}')">
    <img src="/assets/profile.png">
    <div id="credentials">
        <h4>${e[2]}</h4>
        <h5 id="code">${formatUserCode(e[1])}</h5>
    </div>
</div>`;
//             container.innerHTML += `<div class="user" onclick="selectUser(this, ${e[0]})">
//     <img src="/assets/profile.png">
//     <h4>${e[2]}</h4>
// </div>`
        })
    })
}

function toggleTransferResult(state) {
    document.querySelector("#transfer-result").ariaLabel = state ? "open" : "closed";
}

// #region qr related
function toggleQRSection() {
    let e = document.querySelector("#qr-container");

    e.ariaLabel = e.ariaLabel == 'enabled' ? 'disabled' : 'enabled';

    if (e.ariaLabel == 'enabled') {
        startQR();
    } else {
        stopQR();
    }
}

function toggleSelfQR() {
    let e = document.querySelector("#overlay #self-qr");

    e.ariaLabel = e.ariaLabel == 'open' ? 'closed' : 'open';
}

function stopQR() {
    window.scanner.stop();
}

function startQR() {
    window.scanner.start();
    window.updateFlashAvailability();
}

function validateQRCode(code) {
    if (code.length != 8) {
        return;
    }

    toggleQRSection();
    document.querySelector("#destination #query-input input").value = code;
    queryUser();
}
// #endregion
