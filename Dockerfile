ARG NODE_VERSION=18-alpine
FROM node:${NODE_VERSION}

WORKDIR /app

COPY ./package.json ./package-lock.json /app/

RUN npm ci

COPY . /app

CMD ["npm", "start"]
