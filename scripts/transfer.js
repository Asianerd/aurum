var username = fetchLocalStorage("aurum_username");
var password = fetchLocalStorage("aurum_password");

if ((username === null) || (password === null)) {
    window.location.href = "./login.html";
}

document.querySelector("#user-data #username").innerHTML = username;
sendPostRequest(`${BACKEND_ADDRESS}/get_code`, login_info(), (r) => {
    let response = JSON.parse(parseResponse(r));
    document.querySelector("#user-data #code").innerHTML = `#${response.slice(0, 4)}-${response.slice(4, 8)}`;
})

var currencyFormatter = new Intl.NumberFormat('en-UK', {
    style: 'currency',
    currency: 'MYR'
})

function login_info() {
    return JSON.stringify({
        "username":username,
        "password":password
    });
}

var selectedUser = null;
var selectedUserData = ['username', 'code']; // for transfer results
var selectedWallet = 0;

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
    if (selectedUser == null) {
        flag = false;
    }
    let amount = document.querySelector("#transfer-amount input").value;
    if ((amount == '') || (amount == 0)) {
        flag = false;
    }

    document.querySelector("#confirmation").ariaLabel = flag ? "enabled" : "disabled";

    return flag;
}

validateTransfer();

function submitTransfer() {
    if (!validateTransfer()) {
        return;
    }

    let amount = document.querySelector("#transfer-amount input").value;

    // /<from_wallet>/<to_user>/<to_wallet>/<amount>
    var params = `${selectedWallet}/${selectedUser}/0/${amount}`;
    console.log(params);
    sendPostRequest(`${BACKEND_ADDRESS}/wallet/transfer_balance/${params}`, login_info(), (r) => {
        let response = JSON.parse(parseResponse(r));

        let state = response['Ok'];

        console.log(state);

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
}

function displayTransferResult(header, description) {
    document.querySelector("#overlay #transfer-result #header").innerHTML = header;
    document.querySelector("#overlay #transfer-result #description").innerHTML = description;
    document.querySelector("#overlay #transfer-result").ariaLabel = 'open';
}

// #region wallets
fetchWallets();

function fetchWallets() {
    sendPostRequest(`${BACKEND_ADDRESS}/wallet/get_wallets`, login_info(), (r) => {
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

    sendPostRequest(`${BACKEND_ADDRESS}/wallet/get_limit/${selectedWallet}`, login_info(), (r) => {
        let response = JSON.parse(parseResponse(r));

        console.log(response);
    })
}

function toggleWalletChoices() {
    document.querySelector("#wallet-select").ariaLabel = document.querySelector("#wallet-select").ariaLabel == "open" ? "closed" : "open";
}
// #endregion

queryUser();

function queryUser() {
    let container = document.querySelector("#query-result");
    if (!document.querySelector("#query-input input").value) {
        container.innerHTML = '';
        return;
    }

    sendPostRequest(`${BACKEND_ADDRESS}/query_users/${encodeURIComponent(document.querySelector("#query-input input").value)}`, login_info(), (r) => {
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
        <h5 id="code">#${e[1].slice(0, 4)}-${e[1].slice(4, 8)}</h5>
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
