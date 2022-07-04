FROM gcr.io/distroless/nodejs:18

COPY node_modules /app/node_modules
COPY index.js /app/index.js
WORKDIR /app

EXPOSE 3000
CMD ["index.js"]