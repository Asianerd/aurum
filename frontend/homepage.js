// var content_parent;

// content_parent = document.getElementById("content");

// $(document).ready(function() {
//     populate();

//     $('.post').each(function(i, obj) {
//         updateSize($(this), $(this).find('#comment-section'));
//     })
//     $(window).resize(function() {
//         $('.post').each(function(i, obj) {
//             updateSize($(this), $(this).find('#comment-section'));
//         })
//     })

//     setTimeout(function() {
//         $('.post').each(function(i, obj) {
//             updateSize($(this), $(this).find('#comment-section'));
//         })
//     }, 40);
// })

// function updateSize(target, e) {
//     $(e).css('height', $(target).find('#content').outerHeight());
//     //$(e).find('#comment-container').css('height', $(target).find('#content').outerHeight() * 0.8);
// }


// //var post_data = [];
// var post_data = [
//     {
//         username:"Ryan Gosling",
//         description:"Lorem ipsum dolor sit amet consectetur adipisicing elit. Commodi, debitis sapiente. Excepturi fugiat atque placeat quod mollitia dolores quis maiores est sint explicabo voluptatibus, hic cum dolorum maxime perspiciatis qui.",
//         time:"22min",
//         image:"1.png",
//         comments:[
//             {
//                 username:"lasjflasdjf",
//                 description:"Lorem ipsum dolor sit amet consectetur adipisicing elit. Commodi, debitis sapiente. Excepturi fugiat atque placeat quod mollitia dolores quis maiores est sint explicabo voluptatibus, hic cum dolorum maxime perspiciatis qui.",
//                 time:"22min",
//             },
//             {
//                 username:"lasfjklasjdfklasda",
//                 description:"Lorem ipsum dolor sit amet consectetur adipisicing elit. Commodi, debitis sapiente. Excepturi fugiat atque placeat quod mollitia dolores quis maiores est sint explicabo voluptatibus, hic cum dolorum maxime perspiciatis qui.",
//                 time:"22min",
//             },
//             {
//                 username:"Ryan Gosling",
//                 description:"Lorem ipsum dolor sit amet consectetur adipisicing elit. Commodi, debitis sapiente. Excepturi fugiat atque placeat quod mollitia dolores quis maiores est sint explicabo voluptatibus, hic cum dolorum maxime perspiciatis qui.",
//                 time:"22min",
//             },
//             {
//                 username:"Ryan Gosling",
//                 description:"Lorem ipsum dolor sit amet consectetur adipisicing elit. Commodi, debitis sapiente. Excepturi fugiat atque placeat quod mollitia dolores quis maiores est sint explicabo voluptatibus, hic cum dolorum maxime perspiciatis qui.",
//                 time:"22min",
//             },
//             {
//                 username:"Ryan Gosling",
//                 description:"Lorem ipsum dolor sit amet consectetur adipisicing elit. Commodi, debitis sapiente. Excepturi fugiat atque placeat quod mollitia dolores quis maiores est sint explicabo voluptatibus, hic cum dolorum maxime perspiciatis qui.",
//                 time:"22min",
//             },
//             {
//                 username:"Ryan Gosling",
//                 description:"Lorem ipsum dolor sit amet consectetur adipisicing elit. Commodi, debitis sapiente. Excepturi fugiat atque placeat quod mollitia dolores quis maiores est sint explicabo voluptatibus, hic cum dolorum maxime perspiciatis qui.",
//                 time:"22min",
//             }
//         ]
//     },
//     {
//         username:"Ryan Gosling",
//         description:"Lorem ipsum dolor sit amet consectetur adipisicing elit. Commodi, debitis sapiente. Excepturi fugiat atque placeat quod mollitia dolores quis maiores est sint explicabo voluptatibus, hic cum dolorum maxime perspiciatis qui.",
//         time:"22min",
//         image:"2.png",
//         comments:[
//             {
//                 username:"Ryan Gosling",
//                 description:"Lorem ipsum dolor sit amet consectetur adipisicing elit. Commodi, debitis sapiente. Excepturi fugiat atque placeat quod mollitia dolores quis maiores est sint explicabo voluptatibus, hic cum dolorum maxime perspiciatis qui.",
//                 time:"22min",
//             }
//         ]
//     },
//     {
//         username:"Ryan Gosling",
//         description:"Lorem ipsum dolor sit amet consectetur adipisicing elit. Commodi, debitis sapiente. Excepturi fugiat atque placeat quod mollitia dolores quis maiores est sint explicabo voluptatibus, hic cum dolorum maxime perspiciatis qui.",
//         time:"22min",
//         image:"2.png",
//         comments:[

//         ]
//     },
//     {
//         username:"Ryan Gosling",
//         description:"Lorem ipsum dolor sit amet consectetur adipisicing elit. Commodi, debitis sapiente. Excepturi fugiat atque placeat quod mollitia dolores quis maiores est sint explicabo voluptatibus, hic cum dolorum maxime perspiciatis qui.",
//         time:"22min",
//         image:"2.png",
//         comments:[
//             {
//                 username:"Ryan Gosling",
//                 description:"Lorem ipsum dolor sit amet consectetur adipisicing elit. Commodi, debitis sapiente. Excepturi fugiat atque placeat quod mollitia dolores quis maiores est sint explicabo voluptatibus, hic cum dolorum maxime perspiciatis qui.",
//                 time:"22min",
//             },
//             {
//                 username:"Ryan Gosling",
//                 description:"Lorem ipsum dolor sit amet consectetur adipisicing elit. Commodi, debitis sapiente. Excepturi fugiat atque placeat quod mollitia dolores quis maiores est sint explicabo voluptatibus, hic cum dolorum maxime perspiciatis qui.",
//                 time:"22min",
//             },
//             {
//                 username:"Ryan Gosling",
//                 description:"Lorem ipsum dolor sit amet consectetur adipisicing elit. Commodi, debitis sapiente. Excepturi fugiat atque placeat quod mollitia dolores quis maiores est sint explicabo voluptatibus, hic cum dolorum maxime perspiciatis qui.",
//                 time:"22min",
//             },
//             {
//                 username:"Ryan Gosling",
//                 description:"Lorem ipsum dolor sit amet consectetur adipisicing elit. Commodi, debitis sapiente. Excepturi fugiat atque placeat quod mollitia dolores quis maiores est sint explicabo voluptatibus, hic cum dolorum maxime perspiciatis qui.",
//                 time:"22min",
//             },
//             {
//                 username:"Ryan Gosling",
//                 description:"Lorem ipsum dolor sit amet consectetur adipisicing elit. Commodi, debitis sapiente. Excepturi fugiat atque placeat quod mollitia dolores quis maiores est sint explicabo voluptatibus, hic cum dolorum maxime perspiciatis qui.",
//                 time:"22min",
//             },
//             {
//                 username:"Ryan Gosling",
//                 description:"Lorem ipsum dolor sit amet consectetur adipisicing elit. Commodi, debitis sapiente. Excepturi fugiat atque placeat quod mollitia dolores quis maiores est sint explicabo voluptatibus, hic cum dolorum maxime perspiciatis qui.",
//                 time:"22min",
//             }
//         ]
//     }
// ]

// async function fetch_post_data() {
//     //post_data = [];

//     var http = new XMLHttpRequest();
//     http.onreadystatechange = function() {
//         if ((this.readyState == 4) && (this.status == 200)) {
//             // everything went well
//             console.log(post_data);
//             post_data = JSON.parse(JSON.parse(this.responseText));
//             console.log(post_data);
//         }
//     }

//     http.open("GET", `http://127.0.0.1:8000/fetch_all`, true);
//     http.setRequestHeader("Content-type", "application/x-www-form-urlencoded");
//     http.send();
// }

// //fetch_post_data();

// function populate() {
//     content_parent.innerHTML = "";
//     post_data.forEach(element => {
//         var comments = "";
//         if (element['comments'].length == 0) {
//             comments = `<div id="empty">
//                 <h1>
//                     No comments yet :(
//                 </h1>
//                 <h2>
//                     Be the first to say something!
//                 </h2>
//             </div>
//             `;
//         } else {
//             element['comments'].forEach(c => {
//                 comments += `<div id="comment">
//                     <div id="author">
//                         <img src="./profile-images/1.png">
//                         <div id="info">
//                             <div>
//                                 <h1 id="username">
//                                     ${c['username']}
//                                 </h1>
//                                 <h3 id="time">
//                                     •  ${c['time']}
//                                 </h3>
//                             </div>
//                             <div>
//                                 <h2 id="description">
//                                     ${c['description']}
//                                 </h2>
//                             </div>
//                         </div>
//                     </div>
//                 </div>`
//             });
//         }
//         content_parent.innerHTML += `
//     <div class="post">
//         <div id="content">
//             <div id="author">
//                 <img src="./profile-images/1.png">
//                 <div id="info">
//                     <div>
//                         <h1 id="username">
//                             ${element['username']}
//                         </h1>
//                         <h3 id="time">
//                             •  ${element['time']}
//                         </h3>
//                     </div>
//                     <div>
//                         <h2 id="description">
//                             ${element['description']}
//                         </h2>
//                     </div>
//                 </div>
//             </div>
//             ` + (element['image'] ? `<img src="./post-images/${element['image']}">` : '') + `
//         </div>
//         <div id="comment-section">
//             <div id="comment-container">
//                 ${comments}
//             </div>
//             <div id="reply-section">
//                 <input type="text">
//                 <img src="./assets/send.png">
//             </div>
//         </div>
//     </div>
//         `
//     })
// }

$(document).ready(function() {
    populate("highest_growth", document.getElementById('stocks_growth'));
    populate("highest_value", document.getElementById('stocks_value'));
    populate("lowest_growth", document.getElementById('stocks_growth_low'));
    populate("lowest_value", document.getElementById('stocks_value_low'));
})

async function populate(trend, container, amount=5) {
    var http = new XMLHttpRequest();
    http.onreadystatechange = function() {
        if ((this.readyState == 4) && (this.status == 200)) {
            var post_data = JSON.parse(this.responseText);
            container.innerHTML = "";
            post_data.forEach(element => {
                container.innerHTML += stock_as_html(element);
            });
        }
    }

    http.open("GET", `http://127.0.0.1:8000/fetch_stock/${trend}/${amount}`, true);
    http.setRequestHeader("Content-type", "application/x-www-form-urlencoded");
    http.send();
}

async function engine_request() {
    var http = new XMLHttpRequest();
    http.onreadystatechange = function() {
        if ((this.readyState == 4) && (this.status == 200)) {
            // everything went well
            console.log(post_data);
            post_data = JSON.parse(JSON.parse(this.responseText));
            console.log(post_data);
        }
    }

    http.open("GET", `http://127.0.0.1:8000/fetch_all`, true);
    http.setRequestHeader("Content-type", "application/x-www-form-urlencoded");
    http.send();
}

function stock_as_html(element) {
    return `<div class="stock">
        <div id="info">
            <div>
                <h1>${element.name} Inc.</h1>
            </div>
            <div id="pricing">
                <div id="growth" class="${(element['growth_rate'] > 0) ? 'bull' : (element['growth_rate'] == 0 ? 'neutral' : 'bear')}">
                    <img src="./assets/profit.png">
                    <h3>${element.growth_rate.toFixed(2)}%</h3>
                    <img src="./assets/loss.png">
                </div>
                <h2>$${element.history[0].toFixed(2)}</h2>
            </div>
        </div>
        <img src="../aurum_output/${element.name}.svg">
    </div>`
}
