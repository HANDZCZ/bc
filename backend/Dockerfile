FROM scratch

# Expose api port
EXPOSE 80/tcp

# Copy default env file
COPY ./.env.default ./.env

# Copy executable
COPY ./target/x86_64-unknown-linux-musl/release/backend ./backend

# Start
ENTRYPOINT [ "./backend" ]