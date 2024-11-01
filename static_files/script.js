const main = document.querySelector('main');

fetch('/post/get')
    .then(response => response.json())
    .then(data => {
        for (const post of data.posts) {
            const article = document.createElement('article');
            const user_avatar = post.user_avatar !== null ? `<img src="image/${post.user_avatar}" alt="User avatar image">` : '';
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
                <img src="image/${post.post_image}" alt="Posted image">
            `;
            main.appendChild(article);
        }
    })
    .catch(error => {
        console.error('Error fetching posts:', error);
    });
