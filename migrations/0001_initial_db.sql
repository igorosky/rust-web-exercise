CREATE TABLE BlogPosts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_name TEXT NOT NULL,
    content TEXT NOT NULL,
    user_avatar INTEGER NULL DEFAULT NULL,
    post_image INTEGER NULL DEFAULT NULL,
    publication_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(user_avatar) REFERENCES Images(id),
    FOREIGN KEY(post_image) REFERENCES Images(id)
);

CREATE TABLE Images (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    image_hash TEXT NOT NULL,
    image_filename TEXT NOT NULL
);
