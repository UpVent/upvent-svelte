# Etapa de construcción en Node.JS
FROM node:latest as build

# Copia los archivos fuente del proyecto al directorio de trabajo en la imagen de construcción.
WORKDIR /app
COPY . .

# Instala las dependencias y construye la aplicación.
RUN npm install
RUN npm run build

# Etapa de producción
FROM nginx

# Copia el archivo de configuración de nginx.
COPY nginx.conf /etc/nginx/nginx.conf

# Define el directorio de trabajo en la imagen de producción.
WORKDIR /usr/share/nginx/html

# Copia los archivos estáticos desde la etapa de construcción a la imagen de producción.
COPY --from=build /app/build .

# Expone el puerto 80 al host de Docker, para que podamos acceder a la aplicación.
EXPOSE 80

# Ejecuta nginx en primer plano.
CMD ["nginx", "-g", "daemon off;"]
