FROM node:18-alpine
WORKDIR /app

# 1) Copy package files, install deps
COPY package*.json ./
RUN npm install --legacy-peer-deps

EXPOSE 5173
CMD ["npm", "run", "dev"]
