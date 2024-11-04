const GET_POSTS_ENDPOINT = '/post/get_all';

const urlParams = new URLSearchParams(window.location.search);
const error = urlParams.get('error');
if (error !== null) {
    document.getElementById('error-field').removeAttribute('hidden');
    document.getElementById('error-field-message').innerText = error;
}

const main = document.querySelector('section');
function display_posts(posts) {
    for (const post of posts) {
        const article = document.createElement('article');
        const user_avatar = post.user_avatar !== null ? `<img src="image/${post.user_avatar}" alt="User avatar image">` : '';
        const post_image = post.post_image !== null ? `<img src="image/${post.post_image}" alt="Posted image">` : '';
        const date = new Date(post.publication_date);
        const date_locale = date.toLocaleDateString();
        const time_locale = date.toLocaleTimeString();
        article.innerHTML = `
            <div>
                <header>
                    ${user_avatar}
                    <div>
                        <b>${post.user_name}</b> <i>date: ${date_locale} ${time_locale}</i>
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
fetch(GET_POSTS_ENDPOINT)
    .then(response => response.json())
    .then(display_posts)
    .catch(error => {
        console.error('Error fetching posts:', error);
    });
