FROM node:18
WORKDIR /app
RUN apt-get update
RUN apt-get install -y sqlite3 libsqlite3-dev
COPY package*.json tsconfig.json ./
RUN npm i
COPY . .
            