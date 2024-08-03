# Use the latest Node.js image to build the Vue.js application
FROM node:22.4.1-alpine as node_builder

# Set the working directory
WORKDIR /app/vue_frontend

# Copy package.json and package-lock.json
COPY vue_frontend/package*.json ./

# Install dependencies
RUN npm install

# Copy all files and build the Vue.js application
COPY vue_frontend/ .

RUN npm run build

# Use the Rust image to build the Rust application
FROM rust:latest as rust_builder

# Set the working directory
WORKDIR /app

# Copy all files to the build context
COPY . ./

# Build the Rust application
RUN cargo build --release

# Use an Nginx image for the final container
FROM nginx:latest

# Copy Nginx configuration file
COPY vue_frontend/nginx.conf /etc/nginx/conf.d/default.conf

# Set the working directory
WORKDIR /app

# Copy the built Vue.js application from the builder
COPY --from=node_builder /app/vue_frontend/dist /app/public

# Copy the compiled Rust binary from the builder
COPY --from=rust_builder /app/target/release/SecureMq .

# Ensure the binary is executable
RUN chmod +x /app/SecureMq

# Expose port 80
EXPOSE 80

# Start Nginx and the Rust application
CMD ["sh", "-c", "/app/SecureMq & nginx -g 'daemon off;'"]
