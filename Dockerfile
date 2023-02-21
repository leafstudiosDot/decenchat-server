FROM ubuntu:20.04

RUN apt-get update && \
    apt-get install -y curl gnupg && \
    curl -fsSL https://deb.nodesource.com/setup_19.x | bash - && \
    apt-get install -y nodejs postgresql nginx

# PostgreSQL User and Database
USER postgres
RUN /etc/init.d/postgresql start && \
    psql --command "CREATE USER admin WITH SUPERUSER PASSWORD 'admin';" && \
    createdb -O root decensha
USER root
# Set up Nginx
COPY nginx/nginx.conf /etc/nginx/sites-available/default
RUN ln -s /etc/nginx/sites-available/default /etc/nginx/sites-enabled/default

WORKDIR /
COPY package*.json ./
RUN npm install
COPY . .

RUN npm run build

CMD service nginx start && npm start