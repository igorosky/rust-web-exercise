const DEFAULT_LIMIT = 10;
const GET_POSTS_ENDPOINT = '/post/get';



// Pagination initialization
const queryString = window.location.search;
const urlParams = new URLSearchParams(queryString);
let current_limit = urlParams.get('limit') || DEFAULT_LIMIT;
let current_offset = urlParams.get('offset') || 0;
const current_page = current_offset / current_limit + 1;
for(const indicator of document.getElementsByClassName('current-page-indicator')) {
    indicator.innerHTML = current_page;
}
if(current_offset > 0) {
    for(const previous_button of document.getElementsByClassName('previous-page-button')) {
        previous_button.removeAttribute('hidden');
    }
}

function display_next_page_button() {
    for(const previous_button of document.getElementsByClassName('next-page-button')) {
        previous_button.removeAttribute('hidden');
    }
}


const main = document.querySelector('section');
function display_posts(posts) {
    for (const post of posts) {
        const article = document.createElement('article');
        const user_avatar = post.user_avatar !== null ? `<img src="image/${post.user_avatar}" alt="User avatar image">` : '';
        const post_image = post.post_image !== null ? `<img src="image/${post.post_image}" alt="Posted image">` : '';
        article.innerHTML = `
            <div>
                <header>
                    ${user_avatar}
                    <div>
                        <b>${post.user_name}</b> <i>date: ${post.publication_date}</i>
                    </div>
                </header>
                <p>
                    ${post.content}
                </p>
            </div>
            ${post_image}
        `;
        main.appendChild(article);
    }
}

// Post fetching and updating the DOM
fetch(GET_POSTS_ENDPOINT + `?limit=${current_limit}&offset=${current_offset}`)
    .then(response => response.json())
    .then(data => {
        current_limit = data.limit;
        current_offset = data.offset;
        if(data.offset + data.limit < data.total) {
            display_next_page_button();
        }
        display_posts(data.posts);
    })
    .catch(error => {
        console.error('Error fetching posts:', error);
    });

function next_page() {
    window.location.replace(`?limit=${current_limit}&offset=${current_offset + current_limit}`);
}

function previous_page() {
    window.location.replace(`?limit=${current_limit}&offset=${Math.max(0, current_offset - current_limit)}`);
}
