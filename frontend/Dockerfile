# frontend/Dockerfile
FROM node:18-alpine

WORKDIR /app

# copy only package.json so we can cache npm install
COPY package*.json ./
RUN npm install

# now when you mount your code at runtime, node_modules
# in the image won’t get wiped, so you’ll still have npm & deps
EXPOSE 5173

CMD ["npm", "run", "dev"]
