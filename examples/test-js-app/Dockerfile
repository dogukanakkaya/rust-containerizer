
            FROM node:18
            WORKDIR /app
            RUN apt-get update
            RUN apt-get install -y 
            COPY package*.json tsconfig.json ./
            RUN npm i
            COPY . .
            