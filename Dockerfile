FROM rust:1.82.0

WORKDIR /usr/app

ENV DATABASE_URL=/usr/app/db.sqlite3
ENV RUST_LOG=INFO
ENV RUST_BACKTRACE=0
ENV UPLOAD_DIRECTORY=/usr/app/uploads
ENV STATIC_FILES_DIRECTORY=/usr/app/static_files
ENV UPLOAD_BUFFER_SIZE=1024

RUN mkdir -p $UPLOAD_DIRECTORY
RUN mkdir -p $STATIC_FILES_DIRECTORY
    
COPY . src

RUN cargo install --path src
RUN mv src/static_files/* $STATIC_FILES_DIRECTORY
RUN rm -rf src

EXPOSE 3000

CMD ["jet-brains-internships-rust-web"]
