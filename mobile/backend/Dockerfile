# G.A.N.E Backend — Production container
# Build: docker build -t gane-backend .
# Run:   docker run -p 3001:3001 -v gane-data:/app/data gane-backend
FROM node:18-alpine

WORKDIR /app

# Copy source (zero npm dependencies, no install step needed)
COPY package.json db.js server.js ./
COPY routes ./routes

# Persist data outside container in named volume
VOLUME ["/app/data"]

ENV PORT=3001
ENV GANE_DATA_DIR=/app/data
EXPOSE 3001

# Healthcheck against /health endpoint
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD wget --quiet --tries=1 --spider http://localhost:3001/health || exit 1

CMD ["node", "server.js"]
