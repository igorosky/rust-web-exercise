use axum::{response::{Html, IntoResponse}, routing::get, Router};

use super::RouterType;

#[inline]
pub(super) fn initialize() -> RouterType {
    Router::new()
        .route("/home", get(home))
}

async fn home() -> impl IntoResponse {
    Html(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Posts</title>
</head>
    <form method="post" action="post/add" enctype="multipart/form-data">
        <input type="text" name="user_name">
        <input type="text" name="content">
        <input type="text" name="user_avatar_url">
        <input type="file" name="post_image" multiple>
        <input type="submit">
    </form>
    <img src="http://localhost:3000/image/ed63123d-e947-422a-9b36-c97c7406f860" />
<body>
    "#)
}