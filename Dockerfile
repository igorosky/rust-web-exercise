FROM rust:1.86.0

WORKDIR /usr/app

ENV DATABASE_URL=/usr/app/db.sqlite3
ENV RUST_LOG=INFO
ENV RUST_BACKTRACE=0
ENV UPLOAD_DIRECTORY=/usr/app/uploads
ENV STATIC_FILES_DIRECTORY=/usr/app/static_files
ENV UPLOAD_BUFFER_SIZE=10240
ENV MAX_BODY_SIZE=20971520
ENV ADDRESS=0.0.0.0:3000

RUN mkdir -p $UPLOAD_DIRECTORY
RUN mkdir -p $STATIC_FILES_DIRECTORY
    
COPY . src

RUN cargo install --path src
RUN mv src/static_files/* $STATIC_FILES_DIRECTORY
RUN rm -rf src

EXPOSE 3000

CMD ["rust-web-exercise"]
