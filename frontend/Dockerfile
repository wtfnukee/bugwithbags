# Stage 1: Build
FROM node:16 as builder

WORKDIR /app
COPY . .

RUN npm install
RUN npm run build

# Stage 2: Serve
FROM node:16

WORKDIR /app
COPY --from=builder /app/dist /app/dist
RUN npm install -g serve

EXPOSE 80
CMD ["serve", "-s", "dist", "-p", "80"]
