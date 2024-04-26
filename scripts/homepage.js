
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
