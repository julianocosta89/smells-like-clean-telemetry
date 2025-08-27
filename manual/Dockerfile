FROM node:22-alpine

WORKDIR /app

COPY package*.json ./

RUN npm ci --only=production

COPY *.js .

EXPOSE 3000

CMD ["node", "--require", "./instrumentation.js", "app.js"] 
