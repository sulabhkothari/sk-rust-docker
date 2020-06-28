FROM rust:latest
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000
RUN rustup default nightly
ADD src /app
WORKDIR /app
COPY ./Cargo.toml /app
COPY ./src/main.rs /app/src/main.rs
RUN cargo build
COPY start_rocket_with_nginx.sh /app/
COPY domain.crt /app/
COPY domain.rsa /app/
RUN apt-get update \
    && apt-get install -y nginx \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/* \
    && echo "daemon off;" >> /etc/nginx/nginx.conf

EXPOSE 443
#RUN rm /etc/nginx/conf.d/default.conf
RUN rm /etc/nginx/nginx.conf
#COPY default.conf /etc/nginx/conf.d/default.conf
COPY nginx.conf /etc/nginx/nginx.conf
CMD ["sh", "start_rocket_with_nginx.sh"]

#docker build -t hello-rust .
#docker run -d -p 443:443 hello-rust