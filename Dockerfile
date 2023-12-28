# Use an official Rust runtime as a parent image
FROM rust:latest

# Set the working directory in the container
WORKDIR /usr/src/app

# Copy the current directory contents into the container at /usr/src/app
COPY . .

# Print the PATH before adding the Rust binary directory
RUN echo "PATH before: $PATH"

# Build the Rust app for release
RUN cargo install --path .

# Print the PATH after adding the Rust binary directory
RUN echo "PATH after: $PATH"

# List the contents of the Rust binary directory
RUN ls -la /usr/local/cargo/bin

# Add the Rust binary directory to PATH
ENV PATH="/usr/local/cargo/bin:${PATH}"

# When the container launches, run the application
CMD ["itch_plus"]